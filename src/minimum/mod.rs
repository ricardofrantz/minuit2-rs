//! Top-level minimization result.
//!
//! Replaces BasicFunctionMinimum.h / FunctionMinimum.h. Contains the full
//! iteration history, final state, user transformation, and validity flags.

pub mod error;
pub mod gradient;
pub mod parameters;
pub mod seed;
pub mod state;

use seed::MinimumSeed;
use state::MinimumState;

use crate::user_parameter_state::MnUserParameterState;
use crate::user_parameters::MnUserParameters;

/// Result of a minimization.
#[derive(Debug, Clone)]
pub struct FunctionMinimum {
    seed: MinimumSeed,
    states: Vec<MinimumState>,
    up: f64,
    is_above_max_edm: bool,
    reached_call_limit: bool,
    user_state: MnUserParameterState,
}

impl FunctionMinimum {
    pub fn new(seed: MinimumSeed, states: Vec<MinimumState>, up: f64) -> Self {
        // Build user state from the final internal state
        let user_state = Self::build_user_state(&seed, states.last().unwrap_or(seed.state()));

        Self {
            seed,
            states,
            up,
            is_above_max_edm: false,
            reached_call_limit: false,
            user_state,
        }
    }

    /// Create a result that hit the call limit.
    pub fn with_call_limit(seed: MinimumSeed, states: Vec<MinimumState>, up: f64) -> Self {
        let user_state = Self::build_user_state(&seed, states.last().unwrap_or(seed.state()));
        Self {
            seed,
            states,
            up,
            is_above_max_edm: false,
            reached_call_limit: true,
            user_state,
        }
    }

    /// Create a result above max EDM.
    pub fn above_max_edm(seed: MinimumSeed, states: Vec<MinimumState>, up: f64) -> Self {
        let user_state = Self::build_user_state(&seed, states.last().unwrap_or(seed.state()));
        Self {
            seed,
            states,
            up,
            is_above_max_edm: true,
            reached_call_limit: false,
            user_state,
        }
    }

    fn build_user_state(seed: &MinimumSeed, last: &MinimumState) -> MnUserParameterState {
        let trafo = seed.trafo();
        let internal = last.parameters().vec().as_slice();
        let external = trafo.transform(internal);

        // Build MnUserParameters with updated values
        let mut uparams = MnUserParameters::new();
        for (i, p) in trafo.parameters().iter().enumerate() {
            if p.has_limits() {
                uparams.add_limited(p.name(), external[i], p.error(), p.lower_limit(), p.upper_limit());
            } else if p.has_lower_limit() {
                uparams.add_lower_limited(p.name(), external[i], p.error(), p.lower_limit());
            } else if p.has_upper_limit() {
                uparams.add_upper_limited(p.name(), external[i], p.error(), p.upper_limit());
            } else if p.is_const() {
                uparams.add_const(p.name(), p.value());
            } else {
                uparams.add(p.name(), external[i], p.error());
            }
            if p.is_fixed() && !p.is_const() {
                uparams.fix(i);
            }
        }

        let mut state = MnUserParameterState::new(uparams);
        state.set_fval(last.fval());
        state.set_edm(last.edm());
        state.set_nfcn(last.nfcn());
        state.set_valid(true);
        state
    }

    // --- Accessors ---

    pub fn seed(&self) -> &MinimumSeed {
        &self.seed
    }

    pub fn states(&self) -> &[MinimumState] {
        &self.states
    }

    pub fn state(&self) -> &MinimumState {
        self.states.last().unwrap_or_else(|| self.seed.state())
    }

    pub fn user_state(&self) -> &MnUserParameterState {
        &self.user_state
    }

    /// Function value at the minimum.
    pub fn fval(&self) -> f64 {
        self.state().fval()
    }

    /// Estimated distance to minimum.
    pub fn edm(&self) -> f64 {
        self.state().edm()
    }

    /// Total function calls.
    pub fn nfcn(&self) -> usize {
        self.state().nfcn()
    }

    /// Error definition (Up value): 1.0 for chi-square, 0.5 for likelihood.
    pub fn up(&self) -> f64 {
        self.up
    }

    /// Whether the minimization converged properly.
    pub fn is_valid(&self) -> bool {
        self.state().is_valid() && !self.is_above_max_edm && !self.reached_call_limit
    }

    pub fn is_above_max_edm(&self) -> bool {
        self.is_above_max_edm
    }

    pub fn reached_call_limit(&self) -> bool {
        self.reached_call_limit
    }

    /// Parameter values in external (user) space.
    pub fn params(&self) -> Vec<f64> {
        self.seed.trafo().transform(self.state().parameters().vec().as_slice())
    }

    /// Number of variable parameters.
    pub fn n_variable_params(&self) -> usize {
        self.seed.n_variable_params()
    }
}
