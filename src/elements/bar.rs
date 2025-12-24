use egui::{Color32, CornerRadius, Painter, Pos2, Rect, Stroke, StrokeKind};

/// Represents a single bar's geometry and style
/// Mirrors Chart.js BarElement properties
#[derive(Clone, Debug)]
pub struct BarElement {
    /// Center X position
    pub x: f32,
    /// Top Y position (for positive values)
    pub y: f32,
    /// Base Y position (typically axis line)
    pub base: f32,
    /// Bar width
    pub width: f32,
    /// Fill color
    pub fill_color: Color32,
    /// Border color
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
    /// Corner rounding (matches Chart.js borderRadius)
    pub border_radius: CornerRadius,
}

impl BarElement {
    /// Create a new bar element
    pub fn new(x: f32, y: f32, base: f32, width: f32) -> Self {
        Self {
            x,
            y,
            base,
            width,
            fill_color: Color32::from_rgb(54, 162, 235), // Chart.js default blue
            border_color: Color32::TRANSPARENT,
            border_width: 0.0,
            border_radius: CornerRadius::ZERO,
        }
    }

    /// Get the bar's bounding rectangle (full size, not animated)
    pub fn rect(&self) -> Rect {
        let half_width = self.width / 2.0;
        let (top, bottom) = if self.y < self.base {
            (self.y, self.base)
        } else {
            (self.base, self.y)
        };
        Rect::from_min_max(
            Pos2::new(self.x - half_width, top),
            Pos2::new(self.x + half_width, bottom),
        )
    }

    /// Check if point is inside bar (for hit detection)
    /// Mirrors Chart.js inRange(x, y)
    pub fn contains(&self, pos: Pos2) -> bool {
        self.rect().contains(pos)
    }

    /// Get bar height (absolute value)
    pub fn height(&self) -> f32 {
        (self.y - self.base).abs()
    }

    /// Draw the bar with current animation progress
    /// progress: 0.0 = start (no height), 1.0 = full height
    pub fn draw(&self, painter: &Painter, progress: f32) {
        let rect = self.animated_rect(progress);

        // Don't draw if rect has no area
        if rect.width() <= 0.0 || rect.height() <= 0.0 {
            return;
        }

        // Draw fill
        painter.rect_filled(rect, self.border_radius, self.fill_color);

        // Draw border if specified
        if self.border_width > 0.0 && self.border_color != Color32::TRANSPARENT {
            painter.rect_stroke(
                rect,
                self.border_radius,
                Stroke::new(self.border_width, self.border_color),
                StrokeKind::Outside,
            );
        }
    }

    /// Calculate animated rectangle (grows from base)
    pub fn animated_rect(&self, progress: f32) -> Rect {
        let full_rect = self.rect();
        let animated_height = self.height() * progress;

        // Handle zero height case
        if animated_height < 0.5 {
            return Rect::from_min_max(
                Pos2::new(full_rect.min.x, self.base),
                Pos2::new(full_rect.max.x, self.base),
            );
        }

        // Animate from base upward (or downward for negative values)
        if self.y < self.base {
            // Positive value: animate upward from base
            Rect::from_min_max(
                Pos2::new(full_rect.min.x, self.base - animated_height),
                Pos2::new(full_rect.max.x, self.base),
            )
        } else {
            // Negative value: animate downward from base
            Rect::from_min_max(
                Pos2::new(full_rect.min.x, self.base),
                Pos2::new(full_rect.max.x, self.base + animated_height),
            )
        }
    }
}

/// Style configuration for bar elements
#[derive(Clone, Debug)]
pub struct BarStyle {
    /// Colors for each bar (cycles if fewer colors than bars)
    pub fill_colors: Vec<Color32>,
    /// Border color for all bars
    pub border_color: Color32,
    /// Border width
    pub border_width: f32,
    /// Corner rounding
    pub border_radius: CornerRadius,
    /// Bar width as percentage of category width [0.0, 1.0]
    pub bar_percentage: f32,
    /// Category width as percentage of available space [0.0, 1.0]
    pub category_percentage: f32,
}

impl Default for BarStyle {
    fn default() -> Self {
        Self {
            // Chart.js default palette
            fill_colors: vec![
                Color32::from_rgb(54, 162, 235),  // #36a2eb - blue
                Color32::from_rgb(255, 99, 132),  // #ff6384 - red
                Color32::from_rgb(255, 206, 86),  // #ffce56 - yellow
                Color32::from_rgb(75, 192, 192),  // #4bc0c0 - teal
                Color32::from_rgb(153, 102, 255), // #9966ff - purple
                Color32::from_rgb(255, 159, 64),  // #ff9f40 - orange
            ],
            border_color: Color32::TRANSPARENT,
            border_width: 0.0,
            border_radius: CornerRadius::same(4),
            bar_percentage: 0.9,
            category_percentage: 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_rect() {
        let bar = BarElement::new(100.0, 50.0, 100.0, 20.0);
        let rect = bar.rect();

        assert!((rect.min.x - 90.0).abs() < 0.01);
        assert!((rect.max.x - 110.0).abs() < 0.01);
        assert!((rect.min.y - 50.0).abs() < 0.01);
        assert!((rect.max.y - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_bar_contains() {
        let bar = BarElement::new(100.0, 50.0, 100.0, 20.0);

        // Inside bar
        assert!(bar.contains(Pos2::new(100.0, 75.0)));
        assert!(bar.contains(Pos2::new(95.0, 60.0)));

        // Outside bar
        assert!(!bar.contains(Pos2::new(0.0, 0.0)));
        assert!(!bar.contains(Pos2::new(100.0, 110.0)));
        assert!(!bar.contains(Pos2::new(100.0, 40.0)));
        assert!(!bar.contains(Pos2::new(80.0, 75.0)));
    }

    #[test]
    fn test_bar_height() {
        let bar = BarElement::new(100.0, 50.0, 100.0, 20.0);
        assert!((bar.height() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_animated_rect_progress() {
        let bar = BarElement::new(100.0, 50.0, 100.0, 20.0);

        // At progress 0, should be at base
        let rect_0 = bar.animated_rect(0.0);
        assert!(rect_0.height() < 1.0);

        // At progress 0.5, should be half height
        let rect_half = bar.animated_rect(0.5);
        assert!((rect_half.height() - 25.0).abs() < 1.0);

        // At progress 1.0, should be full height
        let rect_full = bar.animated_rect(1.0);
        assert!((rect_full.height() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_negative_bar() {
        // Bar that goes below baseline
        let bar = BarElement::new(100.0, 120.0, 100.0, 20.0);

        assert!((bar.height() - 20.0).abs() < 0.01);

        // Animated rect should grow downward
        let rect_full = bar.animated_rect(1.0);
        assert!(rect_full.min.y < rect_full.max.y);
        assert!((rect_full.min.y - 100.0).abs() < 0.01); // Starts at base
        assert!((rect_full.max.y - 120.0).abs() < 0.01); // Ends at y
    }
}
