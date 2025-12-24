use egui::{Color32, Painter, Pos2, Stroke};

/// Represents a single data point on a line chart
#[derive(Clone, Debug)]
pub struct PointElement {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Point radius
    pub radius: f32,
    /// Fill color
    pub fill_color: Color32,
    /// Border color
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
}

impl PointElement {
    /// Create a new point element
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            radius: 4.0,
            fill_color: Color32::from_rgb(54, 162, 235),
            border_color: Color32::WHITE,
            border_width: 2.0,
        }
    }

    /// Get the point position
    pub fn pos(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }

    /// Check if a position is inside this point (for hit detection)
    pub fn contains(&self, pos: Pos2) -> bool {
        let dx = pos.x - self.x;
        let dy = pos.y - self.y;
        let hit_radius = self.radius + 4.0; // Extra margin for easier clicking
        dx * dx + dy * dy <= hit_radius * hit_radius
    }

    /// Draw the point
    pub fn draw(&self, painter: &Painter) {
        let center = self.pos();

        // Draw border (larger circle behind)
        if self.border_width > 0.0 {
            painter.circle_filled(
                center,
                self.radius + self.border_width,
                self.border_color,
            );
        }

        // Draw fill
        painter.circle_filled(center, self.radius, self.fill_color);
    }

    /// Draw the point with animated Y position
    pub fn draw_animated(&self, painter: &Painter, base_y: f32, progress: f32) {
        let animated_y = base_y + (self.y - base_y) * progress;
        let center = Pos2::new(self.x, animated_y);

        // Draw border (larger circle behind)
        if self.border_width > 0.0 {
            painter.circle_filled(
                center,
                self.radius + self.border_width,
                self.border_color,
            );
        }

        // Draw fill
        painter.circle_filled(center, self.radius, self.fill_color);
    }
}

/// Represents a line connecting multiple points
#[derive(Clone, Debug)]
pub struct LineElement {
    /// Points that make up the line
    pub points: Vec<PointElement>,
    /// Line color
    pub color: Color32,
    /// Line width
    pub width: f32,
    /// Whether to use bezier curves (smooth) or straight lines
    pub curved: bool,
    /// Tension for bezier curves (0.0 = straight, 0.4 = default Chart.js)
    pub tension: f32,
}

impl LineElement {
    /// Create a new line element
    pub fn new(points: Vec<PointElement>) -> Self {
        Self {
            points,
            color: Color32::from_rgb(54, 162, 235),
            width: 2.0,
            curved: true,
            tension: 0.4,
        }
    }

    /// Draw the line
    pub fn draw(&self, painter: &Painter) {
        if self.points.len() < 2 {
            return;
        }

        let positions: Vec<Pos2> = self.points.iter().map(|p| p.pos()).collect();
        self.draw_line_path(painter, &positions);
    }

    /// Draw the line with animated Y positions
    pub fn draw_animated(&self, painter: &Painter, base_y: f32, progress: f32) {
        if self.points.len() < 2 {
            return;
        }

        let positions: Vec<Pos2> = self
            .points
            .iter()
            .map(|p| {
                let animated_y = base_y + (p.y - base_y) * progress;
                Pos2::new(p.x, animated_y)
            })
            .collect();

        self.draw_line_path(painter, &positions);
    }

    /// Draw the actual line path
    fn draw_line_path(&self, painter: &Painter, positions: &[Pos2]) {
        if positions.len() < 2 {
            return;
        }

        let stroke = Stroke::new(self.width, self.color);

        if self.curved && positions.len() > 2 {
            // Draw bezier curves
            self.draw_curved_line(painter, positions, stroke);
        } else {
            // Draw straight line segments
            for i in 0..positions.len() - 1 {
                painter.line_segment([positions[i], positions[i + 1]], stroke);
            }
        }
    }

    /// Draw curved line using quadratic bezier approximation
    fn draw_curved_line(&self, painter: &Painter, positions: &[Pos2], stroke: Stroke) {
        let control_points = self.calculate_control_points(positions);

        // Draw bezier curves between each pair of points
        for i in 0..positions.len() - 1 {
            let p0 = positions[i];
            let p1 = positions[i + 1];
            let (cp1, cp2) = &control_points[i];

            // Approximate cubic bezier with line segments
            self.draw_cubic_bezier(painter, p0, *cp1, *cp2, p1, stroke);
        }
    }

    /// Calculate control points for cubic bezier curves
    fn calculate_control_points(&self, positions: &[Pos2]) -> Vec<(Pos2, Pos2)> {
        let n = positions.len();
        let mut control_points = Vec::with_capacity(n - 1);

        for i in 0..n - 1 {
            let p0 = if i > 0 { positions[i - 1] } else { positions[i] };
            let p1 = positions[i];
            let p2 = positions[i + 1];
            let p3 = if i + 2 < n { positions[i + 2] } else { positions[i + 1] };

            // Calculate control points using Catmull-Rom to Bezier conversion
            let tension = self.tension;

            let cp1 = Pos2::new(
                p1.x + (p2.x - p0.x) * tension / 3.0,
                p1.y + (p2.y - p0.y) * tension / 3.0,
            );

            let cp2 = Pos2::new(
                p2.x - (p3.x - p1.x) * tension / 3.0,
                p2.y - (p3.y - p1.y) * tension / 3.0,
            );

            control_points.push((cp1, cp2));
        }

        control_points
    }

    /// Draw a cubic bezier curve approximated with line segments
    fn draw_cubic_bezier(
        &self,
        painter: &Painter,
        p0: Pos2,
        cp1: Pos2,
        cp2: Pos2,
        p1: Pos2,
        stroke: Stroke,
    ) {
        let segments = 16; // Number of line segments to approximate the curve
        let mut prev = p0;

        for i in 1..=segments {
            let t = i as f32 / segments as f32;
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;

            // Cubic bezier formula
            let x = mt3 * p0.x + 3.0 * mt2 * t * cp1.x + 3.0 * mt * t2 * cp2.x + t3 * p1.x;
            let y = mt3 * p0.y + 3.0 * mt2 * t * cp1.y + 3.0 * mt * t2 * cp2.y + t3 * p1.y;

            let current = Pos2::new(x, y);
            painter.line_segment([prev, current], stroke);
            prev = current;
        }
    }

    /// Draw filled area under the line
    pub fn draw_fill(&self, painter: &Painter, base_y: f32, fill_color: Color32) {
        if self.points.len() < 2 {
            return;
        }

        let positions: Vec<Pos2> = self.points.iter().map(|p| p.pos()).collect();
        self.draw_fill_path(painter, &positions, base_y, fill_color);
    }

    /// Draw filled area with animated Y positions
    pub fn draw_fill_animated(
        &self,
        painter: &Painter,
        base_y: f32,
        progress: f32,
        fill_color: Color32,
    ) {
        if self.points.len() < 2 {
            return;
        }

        let positions: Vec<Pos2> = self
            .points
            .iter()
            .map(|p| {
                let animated_y = base_y + (p.y - base_y) * progress;
                Pos2::new(p.x, animated_y)
            })
            .collect();

        self.draw_fill_path(painter, &positions, base_y, fill_color);
    }

    /// Draw the fill path using triangulation for non-convex shapes
    fn draw_fill_path(&self, painter: &Painter, positions: &[Pos2], base_y: f32, fill_color: Color32) {
        if positions.len() < 2 {
            return;
        }

        // For curved lines, we need to collect all the curve points
        let curve_points = if self.curved && positions.len() > 2 {
            self.collect_curve_points(positions)
        } else {
            positions.to_vec()
        };

        // Draw fill as a series of triangles (fan triangulation from baseline)
        // Each triangle connects: baseline_left, curve_point[i], curve_point[i+1]
        // Plus vertical strips from each curve point to baseline
        use egui::epaint::Mesh;

        let mut mesh = Mesh::default();

        // Add vertices for all curve points and their baseline projections
        for point in &curve_points {
            // Curve point
            mesh.vertices.push(egui::epaint::Vertex {
                pos: *point,
                uv: egui::epaint::WHITE_UV,
                color: fill_color,
            });
            // Baseline point directly below
            mesh.vertices.push(egui::epaint::Vertex {
                pos: Pos2::new(point.x, base_y),
                uv: egui::epaint::WHITE_UV,
                color: fill_color,
            });
        }

        // Create triangles: for each adjacent pair of points, create 2 triangles
        // forming a quad from curve to baseline
        for i in 0..(curve_points.len() - 1) {
            let top_left = (i * 2) as u32;      // curve point i
            let bottom_left = (i * 2 + 1) as u32;  // baseline below i
            let top_right = (i * 2 + 2) as u32; // curve point i+1
            let bottom_right = (i * 2 + 3) as u32; // baseline below i+1

            // Triangle 1: top_left, bottom_left, top_right
            mesh.indices.extend([top_left, bottom_left, top_right]);
            // Triangle 2: top_right, bottom_left, bottom_right
            mesh.indices.extend([top_right, bottom_left, bottom_right]);
        }

        painter.add(egui::Shape::mesh(mesh));
    }

    /// Collect all points along the curved line (including bezier interpolation)
    fn collect_curve_points(&self, positions: &[Pos2]) -> Vec<Pos2> {
        if positions.len() < 3 {
            return positions.to_vec();
        }

        let control_points = self.calculate_control_points(positions);
        let mut all_points = Vec::new();
        let segments = 16;

        for i in 0..positions.len() - 1 {
            let p0 = positions[i];
            let p1 = positions[i + 1];
            let (cp1, cp2) = &control_points[i];

            // Add start point (only for first segment)
            if i == 0 {
                all_points.push(p0);
            }

            // Add bezier interpolation points
            for j in 1..=segments {
                let t = j as f32 / segments as f32;
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;

                let x = mt3 * p0.x + 3.0 * mt2 * t * cp1.x + 3.0 * mt * t2 * cp2.x + t3 * p1.x;
                let y = mt3 * p0.y + 3.0 * mt2 * t * cp1.y + 3.0 * mt * t2 * cp2.y + t3 * p1.y;

                all_points.push(Pos2::new(x, y));
            }
        }

        all_points
    }
}

/// Style configuration for line charts
#[derive(Clone, Debug)]
pub struct LineStyle {
    /// Line color
    pub color: Color32,
    /// Line width
    pub width: f32,
    /// Point radius
    pub point_radius: f32,
    /// Point border width
    pub point_border_width: f32,
    /// Point border color
    pub point_border_color: Color32,
    /// Whether to show points
    pub show_points: bool,
    /// Whether to use curved lines
    pub curved: bool,
    /// Curve tension (0.0-1.0)
    pub tension: f32,
    /// Whether to fill area under line
    pub fill: bool,
    /// Fill color (with alpha for transparency)
    pub fill_color: Option<Color32>,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            color: Color32::from_rgb(54, 162, 235),
            width: 2.0,
            point_radius: 4.0,
            point_border_width: 2.0,
            point_border_color: Color32::WHITE,
            show_points: true,
            curved: true,
            tension: 0.4,
            fill: false,
            fill_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_contains() {
        let point = PointElement::new(100.0, 50.0);

        // Inside point
        assert!(point.contains(Pos2::new(100.0, 50.0)));
        assert!(point.contains(Pos2::new(102.0, 50.0)));

        // Outside point
        assert!(!point.contains(Pos2::new(120.0, 50.0)));
        assert!(!point.contains(Pos2::new(100.0, 70.0)));
    }

    #[test]
    fn test_line_element_creation() {
        let points = vec![
            PointElement::new(0.0, 100.0),
            PointElement::new(50.0, 50.0),
            PointElement::new(100.0, 75.0),
        ];
        let line = LineElement::new(points);

        assert_eq!(line.points.len(), 3);
        assert!(line.curved);
    }

    #[test]
    fn test_control_points_calculation() {
        let points = vec![
            PointElement::new(0.0, 100.0),
            PointElement::new(50.0, 50.0),
            PointElement::new(100.0, 75.0),
        ];
        let line = LineElement::new(points);

        let positions: Vec<Pos2> = line.points.iter().map(|p| p.pos()).collect();
        let control_points = line.calculate_control_points(&positions);

        assert_eq!(control_points.len(), 2); // n-1 control point pairs
    }
}
