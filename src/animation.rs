use egui::Context;
use std::time::Instant;

/// Easing functions matching Chart.js
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Easing {
    Linear,
    #[default]
    EaseOutQuart,
    EaseInQuart,
    EaseInOutQuart,
    EaseOutBounce,
    EaseOutElastic,
    EaseOutCubic,
    EaseInOutCubic,
}

impl Easing {
    /// Apply easing to normalized time t in [0, 1]
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            // Chart.js: 1 - (1 - t)^4
            Easing::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            Easing::EaseInQuart => t.powi(4),
            Easing::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t.powi(4)
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t.powi(3)
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseOutBounce => Self::bounce_out(t),
            Easing::EaseOutElastic => Self::elastic_out(t),
        }
    }

    fn bounce_out(t: f32) -> f32 {
        const N1: f32 = 7.5625;
        const D1: f32 = 2.75;
        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }

    fn elastic_out(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * std::f32::consts::PI) / 3.0;
            2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
        }
    }
}

/// Animation configuration
#[derive(Clone, Debug)]
pub struct AnimationConfig {
    pub easing: Easing,
    pub duration_secs: f32,
    pub enabled: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            easing: Easing::EaseOutQuart,
            duration_secs: 0.8,
            enabled: true,
        }
    }
}

/// Runtime animation state (stored in egui memory)
#[derive(Clone, Debug)]
pub struct AnimationState {
    start_time: Option<Instant>,
    config: AnimationConfig,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            start_time: None,
            config: AnimationConfig::default(),
        }
    }
}

impl AnimationState {
    pub fn new(config: AnimationConfig) -> Self {
        Self {
            start_time: if config.enabled {
                Some(Instant::now())
            } else {
                None
            },
            config,
        }
    }

    /// Restart animation from beginning
    pub fn restart(&mut self) {
        if self.config.enabled {
            self.start_time = Some(Instant::now());
        }
    }

    /// Get current progress [0.0, 1.0] with easing applied
    pub fn progress(&self) -> f32 {
        match self.start_time {
            None => 1.0, // Animation disabled, show final state
            Some(start) => {
                let elapsed = start.elapsed().as_secs_f32();
                let t = (elapsed / self.config.duration_secs).min(1.0);
                self.config.easing.apply(t)
            }
        }
    }

    /// Check if animation is still running
    pub fn is_animating(&self) -> bool {
        match self.start_time {
            None => false,
            Some(start) => start.elapsed().as_secs_f32() < self.config.duration_secs,
        }
    }

    /// Request repaint if animation is active
    pub fn request_repaint_if_animating(&self, ctx: &Context) {
        if self.is_animating() {
            ctx.request_repaint();
        }
    }

    /// Update the animation config (useful for changing duration mid-flight)
    pub fn set_config(&mut self, config: AnimationConfig) {
        self.config = config;
    }
}

/// Builder helper for fluent API
#[derive(Clone, Debug, Default)]
pub struct Animation;

impl Animation {
    /// Create animation with default Chart.js easing (easeOutQuart)
    pub fn ease_out_quart(duration_secs: f32) -> AnimationConfig {
        AnimationConfig {
            easing: Easing::EaseOutQuart,
            duration_secs,
            enabled: true,
        }
    }

    /// Linear animation (constant speed)
    pub fn linear(duration_secs: f32) -> AnimationConfig {
        AnimationConfig {
            easing: Easing::Linear,
            duration_secs,
            enabled: true,
        }
    }

    /// Bouncy animation
    pub fn bounce(duration_secs: f32) -> AnimationConfig {
        AnimationConfig {
            easing: Easing::EaseOutBounce,
            duration_secs,
            enabled: true,
        }
    }

    /// Elastic/springy animation
    pub fn elastic(duration_secs: f32) -> AnimationConfig {
        AnimationConfig {
            easing: Easing::EaseOutElastic,
            duration_secs,
            enabled: true,
        }
    }

    /// Custom easing with specified duration
    pub fn custom(easing: Easing, duration_secs: f32) -> AnimationConfig {
        AnimationConfig {
            easing,
            duration_secs,
            enabled: true,
        }
    }

    /// No animation (instant display)
    pub fn none() -> AnimationConfig {
        AnimationConfig {
            enabled: false,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        for easing in [
            Easing::Linear,
            Easing::EaseOutQuart,
            Easing::EaseInQuart,
            Easing::EaseInOutQuart,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
        ] {
            assert!(
                (easing.apply(0.0) - 0.0).abs() < 0.001,
                "{:?} failed at t=0",
                easing
            );
            assert!(
                (easing.apply(1.0) - 1.0).abs() < 0.001,
                "{:?} failed at t=1",
                easing
            );

            // Mid-values should be in reasonable range
            let mid = easing.apply(0.5);
            assert!(
                mid >= 0.0 && mid <= 1.0,
                "{:?} mid value {} out of range",
                easing,
                mid
            );
        }
    }

    #[test]
    fn test_ease_out_quart_formula() {
        // Chart.js formula: 1 - (1 - t)^4
        let t = 0.5_f32;
        let expected = 1.0 - (1.0 - t).powi(4);
        assert!((Easing::EaseOutQuart.apply(t) - expected).abs() < 0.0001);
    }

    #[test]
    fn test_ease_out_quart_fast_start() {
        // EaseOutQuart should be fast at start, slow at end
        let early = Easing::EaseOutQuart.apply(0.25);
        let late = Easing::EaseOutQuart.apply(0.75);

        // At 25% time, should be more than 25% progress (fast start)
        assert!(early > 0.25);
        // At 75% time, should be close to final (slowing down)
        assert!(late > 0.90);
    }

    #[test]
    fn test_animation_state_progress() {
        let config = AnimationConfig {
            easing: Easing::Linear,
            duration_secs: 1.0,
            enabled: true,
        };
        let state = AnimationState::new(config);

        // Progress should start near 0
        let initial_progress = state.progress();
        assert!(initial_progress < 0.1);

        // Should be animating
        assert!(state.is_animating());
    }

    #[test]
    fn test_animation_disabled() {
        let config = Animation::none();
        let state = AnimationState::new(config);

        // Should immediately be at full progress
        assert!((state.progress() - 1.0).abs() < 0.001);
        assert!(!state.is_animating());
    }

    #[test]
    fn test_bounce_bounds() {
        // Bounce should still end at 1.0
        assert!((Easing::EaseOutBounce.apply(0.0)).abs() < 0.001);
        assert!((Easing::EaseOutBounce.apply(1.0) - 1.0).abs() < 0.001);
    }
}
