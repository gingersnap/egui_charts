use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Map value from one range to another
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    if (in_max - in_min).abs() < f32::EPSILON {
        return out_min;
    }
    let t = (value - in_min) / (in_max - in_min);
    lerp(out_min, out_max, t)
}

/// Calculate nice axis tick values (matches Chart.js behavior)
pub fn nice_ticks(min: f64, max: f64, max_ticks: usize) -> Vec<f64> {
    if min >= max {
        return vec![min];
    }

    let range = max - min;
    let rough_step = range / max_ticks as f64;

    // Find nice step size (1, 2, 5, 10, 20, 50, etc.)
    let magnitude = 10_f64.powf(rough_step.log10().floor());
    let residual = rough_step / magnitude;

    let nice_step = if residual <= 1.5 {
        magnitude
    } else if residual <= 3.0 {
        2.0 * magnitude
    } else if residual <= 7.0 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };

    let start = (min / nice_step).floor() * nice_step;
    let mut ticks = Vec::new();
    let mut tick = start;

    while tick <= max + nice_step * 0.5 {
        if tick >= min - nice_step * 0.001 {
            ticks.push(tick);
        }
        tick += nice_step;
    }

    ticks
}

/// Compute hash of f64 slice for change detection
pub fn compute_data_hash(data: &[f64]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for val in data {
        val.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}

/// Compute hash of string slice for change detection
pub fn compute_labels_hash(labels: &[String]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for label in labels {
        label.hash(&mut hasher);
    }
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_map_range() {
        assert!((map_range(50.0, 0.0, 100.0, 0.0, 1.0) - 0.5).abs() < f32::EPSILON);
        assert!((map_range(0.0, 0.0, 100.0, 0.0, 1.0) - 0.0).abs() < f32::EPSILON);
        assert!((map_range(100.0, 0.0, 100.0, 0.0, 1.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_nice_ticks() {
        let ticks = nice_ticks(0.0, 100.0, 5);
        assert!(!ticks.is_empty());
        assert!(ticks.len() <= 7); // Allow some flexibility
        assert!(ticks[0] <= 0.0);
        assert!(*ticks.last().unwrap() >= 100.0);

        // Check ticks are evenly spaced
        if ticks.len() > 1 {
            let step = ticks[1] - ticks[0];
            for i in 2..ticks.len() {
                let actual_step = ticks[i] - ticks[i - 1];
                assert!((actual_step - step).abs() < 0.001);
            }
        }
    }

    #[test]
    fn test_data_hash_consistency() {
        let data1 = vec![1.0, 2.0, 3.0];
        let data2 = vec![1.0, 2.0, 3.0];
        let data3 = vec![1.0, 2.0, 4.0];

        assert_eq!(compute_data_hash(&data1), compute_data_hash(&data2));
        assert_ne!(compute_data_hash(&data1), compute_data_hash(&data3));
    }

    #[test]
    fn test_data_hash_empty() {
        let empty: Vec<f64> = vec![];
        let hash = compute_data_hash(&empty);
        assert_eq!(hash, compute_data_hash(&empty)); // Consistent for empty
    }
}
