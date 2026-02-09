/// Strategy presets controlling gradient/Hessian calculation effort.
///
/// Three levels matching the C++ `MnStrategy`: low (0), medium (1), high (2).
/// Medium is the default. Constants match MnStrategy.cxx exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MnStrategy {
    strategy: u32,
    grad_ncycles: u32,
    hess_ncycles: u32,
    hess_grad_ncycles: u32,
    grad_step_tol: u32,  // stored as x10 to avoid float equality issues
    grad_tol: u32,       // stored as x100
    hess_step_tol: u32,  // stored as x10
    hess_g2_tol: u32,    // stored as x100
}

impl MnStrategy {
    /// Create a new strategy with the given level (0=low, 1=medium, 2=high).
    pub fn new(level: u32) -> Self {
        let mut s = Self {
            strategy: level,
            grad_ncycles: 0,
            hess_ncycles: 0,
            hess_grad_ncycles: 0,
            grad_step_tol: 0,
            grad_tol: 0,
            hess_step_tol: 0,
            hess_g2_tol: 0,
        };
        match level {
            0 => s.set_low_strategy(),
            2 => s.set_high_strategy(),
            _ => s.set_medium_strategy(),
        }
        s
    }

    fn set_low_strategy(&mut self) {
        self.strategy = 0;
        self.grad_ncycles = 2;
        self.grad_step_tol = 5;   // 0.5
        self.grad_tol = 10;       // 0.1
        self.hess_ncycles = 3;
        self.hess_step_tol = 5;   // 0.5
        self.hess_g2_tol = 10;    // 0.1
        self.hess_grad_ncycles = 1;
    }

    fn set_medium_strategy(&mut self) {
        self.strategy = 1;
        self.grad_ncycles = 3;
        self.grad_step_tol = 3;   // 0.3
        self.grad_tol = 5;        // 0.05
        self.hess_ncycles = 5;
        self.hess_step_tol = 3;   // 0.3
        self.hess_g2_tol = 5;     // 0.05
        self.hess_grad_ncycles = 2;
    }

    fn set_high_strategy(&mut self) {
        self.strategy = 2;
        self.grad_ncycles = 5;
        self.grad_step_tol = 1;   // 0.1
        self.grad_tol = 2;        // 0.02
        self.hess_ncycles = 7;
        self.hess_step_tol = 1;   // 0.1
        self.hess_g2_tol = 2;     // 0.02
        self.hess_grad_ncycles = 6;
    }

    /// Get the strategy level.
    pub fn strategy(&self) -> u32 {
        self.strategy
    }

    /// Get the number of gradient calculation cycles.
    pub fn grad_ncycles(&self) -> u32 {
        self.grad_ncycles
    }

    /// Get the gradient step tolerance.
    pub fn grad_step_tol(&self) -> f64 {
        self.grad_step_tol as f64 / 10.0
    }

    /// Get the gradient tolerance.
    pub fn grad_tol(&self) -> f64 {
        self.grad_tol as f64 / 100.0
    }

    /// Get the number of Hessian calculation cycles.
    pub fn hess_ncycles(&self) -> u32 {
        self.hess_ncycles
    }

    /// Get the Hessian step tolerance.
    pub fn hess_step_tol(&self) -> f64 {
        self.hess_step_tol as f64 / 10.0
    }

    /// Get the Hessian g2 tolerance.
    pub fn hess_g2_tol(&self) -> f64 {
        self.hess_g2_tol as f64 / 100.0
    }

    /// Get the number of Hessian gradient calculation cycles.
    pub fn hess_grad_ncycles(&self) -> u32 {
        self.hess_grad_ncycles
    }

    /// Check if this is a low strategy.
    pub fn is_low(&self) -> bool {
        self.strategy == 0
    }

    /// Check if this is a medium strategy.
    pub fn is_medium(&self) -> bool {
        self.strategy == 1
    }

    /// Check if this is a high strategy.
    pub fn is_high(&self) -> bool {
        self.strategy >= 2
    }
}

impl Default for MnStrategy {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_strategy() {
        let s = MnStrategy::new(0);
        assert!(s.is_low());
        assert_eq!(s.grad_ncycles(), 2);
        assert!((s.grad_step_tol() - 0.5).abs() < 1e-15);
        assert!((s.grad_tol() - 0.1).abs() < 1e-15);
        assert_eq!(s.hess_ncycles(), 3);
        assert_eq!(s.hess_grad_ncycles(), 1);
    }

    #[test]
    fn medium_strategy() {
        let s = MnStrategy::new(1);
        assert!(s.is_medium());
        assert_eq!(s.grad_ncycles(), 3);
        assert!((s.grad_step_tol() - 0.3).abs() < 1e-15);
        assert!((s.grad_tol() - 0.05).abs() < 1e-15);
        assert_eq!(s.hess_ncycles(), 5);
        assert_eq!(s.hess_grad_ncycles(), 2);
    }

    #[test]
    fn high_strategy() {
        let s = MnStrategy::new(2);
        assert!(s.is_high());
        assert_eq!(s.grad_ncycles(), 5);
        assert!((s.grad_step_tol() - 0.1).abs() < 1e-15);
        assert!((s.grad_tol() - 0.02).abs() < 1e-15);
        assert_eq!(s.hess_ncycles(), 7);
        assert_eq!(s.hess_grad_ncycles(), 6);
    }

    #[test]
    fn default_is_medium() {
        let s = MnStrategy::default();
        assert!(s.is_medium());
    }
}
