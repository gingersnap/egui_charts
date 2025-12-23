use egui::{Pos2, Response};

use crate::elements::BarElement;

/// Result of interaction detection
#[derive(Clone, Debug, Default)]
pub struct InteractionResult {
    /// Index of hovered bar (if any)
    pub hovered_index: Option<usize>,
    /// Index of clicked bar (if any)
    pub clicked_index: Option<usize>,
}

/// Evaluate which bar element is being interacted with
/// Mirrors Chart.js evaluateInteractionItems
pub fn evaluate_interaction(bars: &[BarElement], response: &Response) -> InteractionResult {
    let mut result = InteractionResult::default();

    // Check hover
    if let Some(hover_pos) = response.hover_pos() {
        result.hovered_index = find_bar_at_position(bars, hover_pos);
    }

    // Check click
    if response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            result.clicked_index = find_bar_at_position(bars, pos);
        }
    }

    result
}

/// Find bar element at given position
/// Uses Chart.js-style inRange hit testing
fn find_bar_at_position(bars: &[BarElement], pos: Pos2) -> Option<usize> {
    // Iterate in reverse to prioritize bars drawn on top (later in list)
    for (i, bar) in bars.iter().enumerate().rev() {
        if bar.contains(pos) {
            return Some(i);
        }
    }
    None
}

/// Mode for multi-bar interaction
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum InteractionMode {
    /// Highlight single bar under cursor (default)
    #[default]
    Point,
    /// Highlight all bars in same category (x-axis index)
    Index,
    /// Highlight all bars in same dataset
    Dataset,
    /// Highlight nearest bar to cursor
    Nearest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bar_at_position() {
        let bars = vec![
            BarElement::new(50.0, 20.0, 100.0, 30.0),
            BarElement::new(100.0, 30.0, 100.0, 30.0),
            BarElement::new(150.0, 40.0, 100.0, 30.0),
        ];

        // Hit first bar
        assert_eq!(find_bar_at_position(&bars, Pos2::new(50.0, 60.0)), Some(0));

        // Hit second bar
        assert_eq!(find_bar_at_position(&bars, Pos2::new(100.0, 60.0)), Some(1));

        // Hit third bar
        assert_eq!(find_bar_at_position(&bars, Pos2::new(150.0, 60.0)), Some(2));

        // Miss all bars
        assert_eq!(find_bar_at_position(&bars, Pos2::new(200.0, 60.0)), None);
        assert_eq!(find_bar_at_position(&bars, Pos2::new(50.0, 10.0)), None);
    }

    #[test]
    fn test_overlapping_bars_returns_last() {
        // Create overlapping bars
        let bars = vec![
            BarElement::new(100.0, 20.0, 100.0, 60.0), // Wide bar
            BarElement::new(100.0, 30.0, 100.0, 30.0), // Narrower bar on top
        ];

        // Should return the last (top) bar when they overlap
        assert_eq!(find_bar_at_position(&bars, Pos2::new(100.0, 60.0)), Some(1));
    }
}
