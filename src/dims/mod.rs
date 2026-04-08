// Placeholder -- implemented in Task 3
use crate::model::Spin;

/// Dimensions of all input/output arrays for a given family and spin mode.
#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    // Input dimensions
    pub rho: u8,
    pub sigma: u8,
    pub lapl: u8,
    pub tau: u8,
    // Order 0
    pub zk: u8,
    // Order 1
    pub vrho: u8,
    pub vsigma: u8,
    pub vlapl: u8,
    pub vtau: u8,
    // Order 2
    pub v2rho2: u8,
    pub v2rhosigma: u8,
    pub v2rholapl: u8,
    pub v2rhotau: u8,
    pub v2sigma2: u8,
    pub v2sigmalapl: u8,
    pub v2sigmatau: u8,
    pub v2lapl2: u8,
    pub v2lapltau: u8,
    pub v2tau2: u8,
    // Order 3
    pub v3rho3: u16,
    pub v3rho2sigma: u16,
    pub v3rho2lapl: u16,
    pub v3rho2tau: u16,
    pub v3rhosigma2: u16,
    pub v3rhosigmalapl: u16,
    pub v3rhosigmatau: u16,
    pub v3rholapl2: u16,
    pub v3rholapltau: u16,
    pub v3rhotau2: u16,
    pub v3sigma3: u16,
    pub v3sigma2lapl: u16,
    pub v3sigma2tau: u16,
    pub v3sigmalapl2: u16,
    pub v3sigmalapltau: u16,
    pub v3sigmatau2: u16,
    pub v3lapl3: u16,
    pub v3lapl2tau: u16,
    pub v3lapltau2: u16,
    pub v3tau3: u16,
    // Order 4
    pub v4rho4: u16,
    pub v4rho3sigma: u16,
    pub v4rho3lapl: u16,
    pub v4rho3tau: u16,
    pub v4rho2sigma2: u16,
    pub v4rho2sigmalapl: u16,
    pub v4rho2sigmatau: u16,
    pub v4rho2lapl2: u16,
    pub v4rho2lapltau: u16,
    pub v4rho2tau2: u16,
    pub v4rhosigma3: u16,
    pub v4rhosigma2lapl: u16,
    pub v4rhosigma2tau: u16,
    pub v4rhosigmalapl2: u16,
    pub v4rhosigmalapltau: u16,
    pub v4rhosigmatau2: u16,
    pub v4rholapl3: u16,
    pub v4rholapl2tau: u16,
    pub v4rholapltau2: u16,
    pub v4rhotau3: u16,
    pub v4sigma4: u16,
    pub v4sigma3lapl: u16,
    pub v4sigma3tau: u16,
    pub v4sigma2lapl2: u16,
    pub v4sigma2lapltau: u16,
    pub v4sigma2tau2: u16,
    pub v4sigmalapl3: u16,
    pub v4sigmalapl2tau: u16,
    pub v4sigmalapltau2: u16,
    pub v4sigmatau3: u16,
    pub v4lapl4: u16,
    pub v4lapl3tau: u16,
    pub v4lapl2tau2: u16,
    pub v4lapltau3: u16,
    pub v4tau4: u16,
}

impl Dimensions {
    pub fn lda(_spin: Spin) -> Self {
        Self::zeroed()
    }

    pub fn gga(_spin: Spin) -> Self {
        Self::zeroed()
    }

    pub fn mgga(_spin: Spin) -> Self {
        Self::zeroed()
    }

    pub fn total_output_components(&self) -> usize {
        0
    }

    fn zeroed() -> Self {
        // Safety: all fields are numeric, zero is valid
        unsafe { std::mem::zeroed() }
    }
}
