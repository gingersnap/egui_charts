use egui::{Color32, Painter, Pos2, Stroke};
use std::f32::consts::PI;

/// Represents an arc segment for pie/donut charts
#[derive(Clone, Debug)]
pub struct ArcElement {
    /// Center position
    pub center: Pos2,
    /// Inner radius (0 for pie, >0 for donut)
    pub inner_radius: f32,
    /// Outer radius
    pub outer_radius: f32,
    /// Start angle in radians (0 = right, PI/2 = bottom)
    pub start_angle: f32,
    /// End angle in radians
    pub end_angle: f32,
    /// Fill color
    pub fill_color: Color32,
    /// Border color
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
}

impl ArcElement {
    /// Create a new arc element
    pub fn new(
        center: Pos2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        Self {
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            fill_color: Color32::from_rgb(54, 162, 235),
            border_color: Color32::WHITE,
            border_width: 2.0,
        }
    }

    /// Check if a point is inside this arc segment
    pub fn contains(&self, pos: Pos2) -> bool {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Check radius bounds
        if distance < self.inner_radius || distance > self.outer_radius {
            return false;
        }

        // Check angle bounds
        let mut angle = dy.atan2(dx);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }

        // Normalize angles to 0..2PI range
        let start = normalize_angle(self.start_angle);
        let end = normalize_angle(self.end_angle);

        if start <= end {
            angle >= start && angle <= end
        } else {
            // Arc crosses 0/2PI boundary
            angle >= start || angle <= end
        }
    }

    /// Get the middle angle of the arc
    pub fn mid_angle(&self) -> f32 {
        (self.start_angle + self.end_angle) / 2.0
    }

    /// Get a point at the middle of the arc at a given radius
    pub fn mid_point(&self, radius: f32) -> Pos2 {
        let angle = self.mid_angle();
        Pos2::new(
            self.center.x + angle.cos() * radius,
            self.center.y + angle.sin() * radius,
        )
    }

    /// Draw the arc segment
    pub fn draw(&self, painter: &Painter) {
        self.draw_arc(painter, self.start_angle, self.end_angle);
    }

    /// Draw the arc with animated end angle
    pub fn draw_animated(&self, painter: &Painter, progress: f32) {
        let animated_end = self.start_angle + (self.end_angle - self.start_angle) * progress;
        self.draw_arc(painter, self.start_angle, animated_end);
    }

    /// Draw arc between given angles
    fn draw_arc(&self, painter: &Painter, start: f32, end: f32) {
        if (end - start).abs() < 0.001 {
            return;
        }

        // Build polygon points for the arc
        let segments = ((end - start).abs() * 32.0 / PI).max(8.0) as usize;
        let mut points = Vec::with_capacity(segments * 2 + 2);

        // Outer arc (clockwise)
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start + (end - start) * t;
            points.push(Pos2::new(
                self.center.x + angle.cos() * self.outer_radius,
                self.center.y + angle.sin() * self.outer_radius,
            ));
        }

        // Inner arc (counter-clockwise) or center point
        if self.inner_radius > 0.0 {
            for i in (0..=segments).rev() {
                let t = i as f32 / segments as f32;
                let angle = start + (end - start) * t;
                points.push(Pos2::new(
                    self.center.x + angle.cos() * self.inner_radius,
                    self.center.y + angle.sin() * self.inner_radius,
                ));
            }
        } else {
            // For pie (no hole), add center point
            points.push(self.center);
        }

        // Draw filled polygon
        if points.len() >= 3 {
            painter.add(egui::Shape::convex_polygon(
                points.clone(),
                self.fill_color,
                Stroke::NONE,
            ));

            // Draw border
            if self.border_width > 0.0 {
                // Draw outer arc border
                let outer_points: Vec<Pos2> = (0..=segments)
                    .map(|i| {
                        let t = i as f32 / segments as f32;
                        let angle = start + (end - start) * t;
                        Pos2::new(
                            self.center.x + angle.cos() * self.outer_radius,
                            self.center.y + angle.sin() * self.outer_radius,
                        )
                    })
                    .collect();

                for i in 0..outer_points.len() - 1 {
                    painter.line_segment(
                        [outer_points[i], outer_points[i + 1]],
                        Stroke::new(self.border_width, self.border_color),
                    );
                }

                // Draw radial lines at start and end
                let start_outer = Pos2::new(
                    self.center.x + start.cos() * self.outer_radius,
                    self.center.y + start.sin() * self.outer_radius,
                );
                let start_inner = if self.inner_radius > 0.0 {
                    Pos2::new(
                        self.center.x + start.cos() * self.inner_radius,
                        self.center.y + start.sin() * self.inner_radius,
                    )
                } else {
                    self.center
                };
                painter.line_segment(
                    [start_inner, start_outer],
                    Stroke::new(self.border_width, self.border_color),
                );

                let end_outer = Pos2::new(
                    self.center.x + end.cos() * self.outer_radius,
                    self.center.y + end.sin() * self.outer_radius,
                );
                let end_inner = if self.inner_radius > 0.0 {
                    Pos2::new(
                        self.center.x + end.cos() * self.inner_radius,
                        self.center.y + end.sin() * self.inner_radius,
                    )
                } else {
                    self.center
                };
                painter.line_segment(
                    [end_inner, end_outer],
                    Stroke::new(self.border_width, self.border_color),
                );

                // Draw inner arc border (for donut)
                if self.inner_radius > 0.0 {
                    let inner_points: Vec<Pos2> = (0..=segments)
                        .map(|i| {
                            let t = i as f32 / segments as f32;
                            let angle = start + (end - start) * t;
                            Pos2::new(
                                self.center.x + angle.cos() * self.inner_radius,
                                self.center.y + angle.sin() * self.inner_radius,
                            )
                        })
                        .collect();

                    for i in 0..inner_points.len() - 1 {
                        painter.line_segment(
                            [inner_points[i], inner_points[i + 1]],
                            Stroke::new(self.border_width, self.border_color),
                        );
                    }
                }
            }
        }
    }
}

/// Normalize angle to 0..2PI range
fn normalize_angle(angle: f32) -> f32 {
    let mut a = angle % (2.0 * PI);
    if a < 0.0 {
        a += 2.0 * PI;
    }
    a
}

/// Style for pie/donut charts
#[derive(Clone, Debug)]
pub struct PieStyle {
    /// Colors for each segment
    pub colors: Vec<Color32>,
    /// Border color between segments
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
    /// Donut hole ratio (0.0 = pie, 0.5 = half radius hole)
    pub donut_ratio: f32,
    /// Start angle in radians (default: -PI/2 = top)
    pub start_angle: f32,
}

impl Default for PieStyle {
    fn default() -> Self {
        Self {
            colors: vec![
                Color32::from_rgb(54, 162, 235),  // blue
                Color32::from_rgb(255, 99, 132),  // red
                Color32::from_rgb(255, 206, 86),  // yellow
                Color32::from_rgb(75, 192, 192),  // teal
                Color32::from_rgb(153, 102, 255), // purple
                Color32::from_rgb(255, 159, 64),  // orange
            ],
            border_color: Color32::WHITE,
            border_width: 2.0,
            donut_ratio: 0.0,
            start_angle: -PI / 2.0, // Start from top
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_contains_radius() {
        let arc = ArcElement::new(
            Pos2::new(100.0, 100.0),
            20.0,  // inner
            50.0,  // outer
            0.0,   // start
            PI,    // end (half circle)
        );

        // Inside arc (right side, within radius)
        assert!(arc.contains(Pos2::new(130.0, 100.0)));

        // Outside arc (too far)
        assert!(!arc.contains(Pos2::new(160.0, 100.0)));

        // Inside hole
        assert!(!arc.contains(Pos2::new(110.0, 100.0)));

        // Wrong angle (top side, arc covers bottom half 0 to PI)
        assert!(!arc.contains(Pos2::new(100.0, 70.0)));
    }

    #[test]
    fn test_mid_point() {
        let arc = ArcElement::new(
            Pos2::new(100.0, 100.0),
            0.0,
            50.0,
            0.0,
            PI / 2.0, // Quarter circle
        );

        let mid = arc.mid_point(50.0);
        // Mid angle is PI/4, so point should be at 45 degrees
        let expected_x = 100.0 + 50.0 * (PI / 4.0).cos();
        let expected_y = 100.0 + 50.0 * (PI / 4.0).sin();

        assert!((mid.x - expected_x).abs() < 0.01);
        assert!((mid.y - expected_y).abs() < 0.01);
    }

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(0.0) - 0.0).abs() < 0.001);
        assert!((normalize_angle(2.0 * PI) - 0.0).abs() < 0.001);
        assert!((normalize_angle(-PI / 2.0) - (3.0 * PI / 2.0)).abs() < 0.001);
    }
}
