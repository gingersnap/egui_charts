use egui::{Color32, CornerRadius, FontId, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};

/// Tooltip configuration
#[derive(Clone, Debug)]
pub struct TooltipConfig {
    pub enabled: bool,
    pub background_color: Color32,
    pub text_color: Color32,
    pub border_color: Color32,
    pub border_width: f32,
    pub border_radius: CornerRadius,
    pub padding: Vec2,
    pub font_size: f32,
}

impl Default for TooltipConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            background_color: Color32::from_rgba_unmultiplied(0, 0, 0, 220),
            text_color: Color32::WHITE,
            border_color: Color32::from_gray(100),
            border_width: 0.0,
            border_radius: CornerRadius::same(4),
            padding: Vec2::new(10.0, 8.0),
            font_size: 13.0,
        }
    }
}

/// Tooltip content
#[derive(Clone, Debug)]
pub struct TooltipContent {
    pub title: Option<String>,
    pub label: String,
    pub value: String,
    pub color: Color32,
}

/// Calculate tooltip position with collision detection
/// Mirrors Chart.js tooltip positioning logic
pub fn calculate_tooltip_position(anchor: Pos2, tooltip_size: Vec2, chart_bounds: Rect) -> Pos2 {
    let margin = 12.0;

    // Default: position above and centered on anchor
    let mut x = anchor.x - tooltip_size.x / 2.0;
    let mut y = anchor.y - tooltip_size.y - margin;

    // Horizontal bounds check
    if x < chart_bounds.min.x + margin {
        x = chart_bounds.min.x + margin;
    } else if x + tooltip_size.x > chart_bounds.max.x - margin {
        x = chart_bounds.max.x - tooltip_size.x - margin;
    }

    // Vertical bounds check: flip below if no room above
    if y < chart_bounds.min.y + margin {
        y = anchor.y + margin; // Position below anchor
    }

    Pos2::new(x, y)
}

/// Draw tooltip with content
pub fn draw_tooltip(
    painter: &Painter,
    content: &TooltipContent,
    position: Pos2,
    config: &TooltipConfig,
) {
    let font_id = FontId::proportional(config.font_size);

    // Calculate text layout
    let label_text = format!("{}: ", content.label);
    let galley_label = painter.layout_no_wrap(label_text.clone(), font_id.clone(), config.text_color);
    let galley_value = painter.layout_no_wrap(content.value.clone(), font_id.clone(), config.text_color);

    let text_width = galley_label.size().x + galley_value.size().x;
    let text_height = galley_label.size().y.max(galley_value.size().y);

    // Color indicator size
    let indicator_size = 10.0;
    let indicator_margin = 8.0;

    // Calculate background rect
    let bg_rect = Rect::from_min_size(
        position,
        Vec2::new(
            indicator_size + indicator_margin + text_width + config.padding.x * 2.0,
            text_height + config.padding.y * 2.0,
        ),
    );

    // Draw shadow (subtle)
    let shadow_offset = Vec2::new(2.0, 2.0);
    let shadow_rect = bg_rect.translate(shadow_offset);
    painter.rect_filled(
        shadow_rect,
        config.border_radius,
        Color32::from_rgba_unmultiplied(0, 0, 0, 40),
    );

    // Draw background
    painter.rect_filled(bg_rect, config.border_radius, config.background_color);

    // Draw border
    if config.border_width > 0.0 {
        painter.rect_stroke(
            bg_rect,
            config.border_radius,
            Stroke::new(config.border_width, config.border_color),
            StrokeKind::Outside,
        );
    }

    // Draw color indicator (small square)
    let indicator_rect = Rect::from_min_size(
        Pos2::new(
            bg_rect.min.x + config.padding.x,
            bg_rect.center().y - indicator_size / 2.0,
        ),
        Vec2::splat(indicator_size),
    );
    painter.rect_filled(indicator_rect, CornerRadius::same(2), content.color);

    // Draw text
    let text_x = indicator_rect.max.x + indicator_margin;
    let text_y = bg_rect.min.y + config.padding.y;

    painter.galley(Pos2::new(text_x, text_y), galley_label, config.text_color);
    painter.galley(
        Pos2::new(
            text_x
                + painter
                    .layout_no_wrap(label_text, font_id, config.text_color)
                    .size()
                    .x,
            text_y,
        ),
        galley_value,
        config.text_color,
    );
}

/// Measure tooltip size for positioning calculations
pub fn measure_tooltip_size(
    painter: &Painter,
    content: &TooltipContent,
    config: &TooltipConfig,
) -> Vec2 {
    let font_id = FontId::proportional(config.font_size);

    let label_text = format!("{}: ", content.label);
    let galley_label = painter.layout_no_wrap(label_text, font_id.clone(), config.text_color);
    let galley_value = painter.layout_no_wrap(content.value.clone(), font_id, config.text_color);

    let text_width = galley_label.size().x + galley_value.size().x;
    let text_height = galley_label.size().y.max(galley_value.size().y);

    let indicator_size = 10.0;
    let indicator_margin = 8.0;

    Vec2::new(
        indicator_size + indicator_margin + text_width + config.padding.x * 2.0,
        text_height + config.padding.y * 2.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_position_centered() {
        let anchor = Pos2::new(200.0, 100.0);
        let size = Vec2::new(100.0, 30.0);
        let bounds = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(400.0, 300.0));

        let pos = calculate_tooltip_position(anchor, size, bounds);

        // Should be centered horizontally above anchor
        assert!((pos.x - 150.0).abs() < 1.0); // 200 - 100/2
        assert!(pos.y < anchor.y); // Above anchor
    }

    #[test]
    fn test_tooltip_position_left_collision() {
        let anchor = Pos2::new(30.0, 100.0); // Near left edge
        let size = Vec2::new(100.0, 30.0);
        let bounds = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(400.0, 300.0));

        let pos = calculate_tooltip_position(anchor, size, bounds);

        // Should not go past left edge
        assert!(pos.x >= bounds.min.x);
    }

    #[test]
    fn test_tooltip_position_right_collision() {
        let anchor = Pos2::new(370.0, 100.0); // Near right edge
        let size = Vec2::new(100.0, 30.0);
        let bounds = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(400.0, 300.0));

        let pos = calculate_tooltip_position(anchor, size, bounds);

        // Should not go past right edge
        assert!(pos.x + size.x <= bounds.max.x);
    }

    #[test]
    fn test_tooltip_position_top_collision() {
        let anchor = Pos2::new(200.0, 20.0); // Near top edge
        let size = Vec2::new(100.0, 30.0);
        let bounds = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(400.0, 300.0));

        let pos = calculate_tooltip_position(anchor, size, bounds);

        // Should flip below anchor
        assert!(pos.y > anchor.y);
    }
}
