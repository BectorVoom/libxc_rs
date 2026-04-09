use crate::model::Spin;

/// Dimensions of all input/output arrays for a given family and spin mode.
/// Used to validate buffer sizes and compute strides.
///
/// Values are derived from libxc `util.c` `internal_counters_set_{lda,gga,mgga}`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions {
    // Input dimensions (elements per grid point)
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
    // Order 3 (u16: polarized MGGA values can exceed 255)
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
    // Order 4 (u16: polarized MGGA order 4 reaches large values)
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
    /// LDA dimensions: only rho derivatives.
    /// Matches libxc `internal_counters_set_lda`.
    pub fn lda(spin: Spin) -> Self {
        let mut d = Self::zeroed();
        d.rho = spin as u8;
        d.vrho = spin as u8;
        d.zk = 1;
        match spin {
            Spin::Unpolarized => {
                d.v2rho2 = 1;
                d.v3rho3 = 1;
                d.v4rho4 = 1;
            }
            Spin::Polarized => {
                d.v2rho2 = 3;
                d.v3rho3 = 4;
                d.v4rho4 = 5;
            }
        }
        d
    }

    /// GGA dimensions: adds sigma derivatives.
    /// Matches libxc `internal_counters_set_gga`.
    pub fn gga(spin: Spin) -> Self {
        let mut d = Self::lda(spin);
        match spin {
            Spin::Unpolarized => {
                d.sigma = 1;
                d.vsigma = 1;
                d.v2rhosigma = 1;
                d.v2sigma2 = 1;
                d.v3rho2sigma = 1;
                d.v3rhosigma2 = 1;
                d.v3sigma3 = 1;
                d.v4rho3sigma = 1;
                d.v4rho2sigma2 = 1;
                d.v4rhosigma3 = 1;
                d.v4sigma4 = 1;
            }
            Spin::Polarized => {
                d.sigma = 3;
                d.vsigma = 3;
                d.v2rhosigma = 2 * 3;
                d.v2sigma2 = 6;
                d.v3rho2sigma = 3 * 3;
                d.v3rhosigma2 = 2 * 6;
                d.v3sigma3 = 10;
                d.v4rho3sigma = 4 * 3;
                d.v4rho2sigma2 = 3 * 6;
                d.v4rhosigma3 = 2 * 10;
                d.v4sigma4 = 15;
            }
        }
        d
    }

    /// MGGA dimensions: adds laplacian and tau cross-derivatives.
    /// Matches libxc `internal_counters_set_mgga`.
    pub fn mgga(spin: Spin) -> Self {
        let mut d = Self::gga(spin);
        d.lapl = spin as u8;
        d.vlapl = spin as u8;
        d.tau = spin as u8;
        d.vtau = spin as u8;
        match spin {
            Spin::Unpolarized => {
                // Order 2
                d.v2rholapl = 1;
                d.v2rhotau = 1;
                d.v2sigmalapl = 1;
                d.v2sigmatau = 1;
                d.v2lapl2 = 1;
                d.v2lapltau = 1;
                d.v2tau2 = 1;
                // Order 3
                d.v3rho2lapl = 1;
                d.v3rho2tau = 1;
                d.v3rhosigmalapl = 1;
                d.v3rhosigmatau = 1;
                d.v3rholapl2 = 1;
                d.v3rholapltau = 1;
                d.v3rhotau2 = 1;
                d.v3sigma2lapl = 1;
                d.v3sigma2tau = 1;
                d.v3sigmalapl2 = 1;
                d.v3sigmalapltau = 1;
                d.v3sigmatau2 = 1;
                d.v3lapl3 = 1;
                d.v3lapl2tau = 1;
                d.v3lapltau2 = 1;
                d.v3tau3 = 1;
                // Order 4
                d.v4rho3lapl = 1;
                d.v4rho3tau = 1;
                d.v4rho2sigmalapl = 1;
                d.v4rho2sigmatau = 1;
                d.v4rho2lapl2 = 1;
                d.v4rho2lapltau = 1;
                d.v4rho2tau2 = 1;
                d.v4rhosigma2lapl = 1;
                d.v4rhosigma2tau = 1;
                d.v4rhosigmalapl2 = 1;
                d.v4rhosigmalapltau = 1;
                d.v4rhosigmatau2 = 1;
                d.v4rholapl3 = 1;
                d.v4rholapl2tau = 1;
                d.v4rholapltau2 = 1;
                d.v4rhotau3 = 1;
                d.v4sigma3lapl = 1;
                d.v4sigma3tau = 1;
                d.v4sigma2lapl2 = 1;
                d.v4sigma2lapltau = 1;
                d.v4sigma2tau2 = 1;
                d.v4sigmalapl3 = 1;
                d.v4sigmalapl2tau = 1;
                d.v4sigmalapltau2 = 1;
                d.v4sigmatau3 = 1;
                d.v4lapl4 = 1;
                d.v4lapl3tau = 1;
                d.v4lapl2tau2 = 1;
                d.v4lapltau3 = 1;
                d.v4tau4 = 1;
            }
            Spin::Polarized => {
                // Order 2 -- values from libxc util.c
                d.v2rholapl = 2 * 2;      // 4
                d.v2rhotau = 2 * 2;        // 4
                d.v2sigmalapl = 3 * 2;     // 6
                d.v2sigmatau = 3 * 2;      // 6
                d.v2lapl2 = 3;
                d.v2lapltau = 2 * 2;       // 4
                d.v2tau2 = 3;
                // Order 3
                d.v3rho2lapl = 3 * 2;      // 6
                d.v3rho2tau = 3 * 2;        // 6
                d.v3rhosigmalapl = 2 * 3 * 2; // 12
                d.v3rhosigmatau = 2 * 3 * 2;  // 12
                d.v3rholapl2 = 2 * 3;      // 6
                d.v3rholapltau = 2 * 2 * 2; // 8
                d.v3rhotau2 = 2 * 3;        // 6
                d.v3sigma2lapl = 6 * 2;    // 12
                d.v3sigma2tau = 6 * 2;      // 12
                d.v3sigmalapl2 = 3 * 3;    // 9
                d.v3sigmalapltau = 3 * 2 * 2; // 12
                d.v3sigmatau2 = 3 * 3;      // 9
                d.v3lapl3 = 4;
                d.v3lapl2tau = 3 * 2;      // 6
                d.v3lapltau2 = 2 * 3;       // 6
                d.v3tau3 = 4;
                // Order 4
                d.v4rho3lapl = 4 * 2;          // 8
                d.v4rho3tau = 4 * 2;            // 8
                d.v4rho2sigmalapl = 3 * 3 * 2;  // 18
                d.v4rho2sigmatau = 3 * 3 * 2;   // 18
                d.v4rho2lapl2 = 3 * 3;          // 9
                d.v4rho2lapltau = 3 * 2 * 2;    // 12
                d.v4rho2tau2 = 3 * 3;            // 9
                d.v4rhosigma2lapl = 3 * 6 * 2;  // 36
                d.v4rhosigma2tau = 3 * 6 * 2;    // 36
                d.v4rhosigmalapl2 = 2 * 3 * 3;  // 18
                d.v4rhosigmalapltau = 2 * 3 * 2 * 2; // 24
                d.v4rhosigmatau2 = 2 * 6 * 3;   // 36
                d.v4rholapl3 = 2 * 4;            // 8
                d.v4rholapl2tau = 2 * 3 * 2;     // 12
                d.v4rholapltau2 = 2 * 2 * 3;     // 12
                d.v4rhotau3 = 2 * 4;              // 8
                d.v4sigma3lapl = 10 * 2;          // 20
                d.v4sigma3tau = 10 * 3;            // 30
                d.v4sigma2lapl2 = 6 * 3;          // 18
                d.v4sigma2lapltau = 6 * 2 * 2;    // 24
                d.v4sigma2tau2 = 6 * 3;            // 18
                d.v4sigmalapl3 = 3 * 4;            // 12
                d.v4sigmalapl2tau = 3 * 3 * 2;     // 18
                d.v4sigmalapltau2 = 3 * 2 * 3;     // 18
                d.v4sigmatau3 = 3 * 4;              // 12
                d.v4lapl4 = 5;
                d.v4lapl3tau = 4 * 2;              // 8
                d.v4lapl2tau2 = 3 * 3;              // 9
                d.v4lapltau3 = 2 * 4;               // 8
                d.v4tau4 = 5;
            }
        }
        d
    }

    /// Sum of all output dimension fields (all derivative orders).
    pub fn total_output_components(&self) -> usize {
        let o0 = self.zk as usize;
        let o1 = self.vrho as usize
            + self.vsigma as usize
            + self.vlapl as usize
            + self.vtau as usize;
        let o2 = self.v2rho2 as usize
            + self.v2rhosigma as usize
            + self.v2rholapl as usize
            + self.v2rhotau as usize
            + self.v2sigma2 as usize
            + self.v2sigmalapl as usize
            + self.v2sigmatau as usize
            + self.v2lapl2 as usize
            + self.v2lapltau as usize
            + self.v2tau2 as usize;
        let o3 = self.v3rho3 as usize
            + self.v3rho2sigma as usize
            + self.v3rho2lapl as usize
            + self.v3rho2tau as usize
            + self.v3rhosigma2 as usize
            + self.v3rhosigmalapl as usize
            + self.v3rhosigmatau as usize
            + self.v3rholapl2 as usize
            + self.v3rholapltau as usize
            + self.v3rhotau2 as usize
            + self.v3sigma3 as usize
            + self.v3sigma2lapl as usize
            + self.v3sigma2tau as usize
            + self.v3sigmalapl2 as usize
            + self.v3sigmalapltau as usize
            + self.v3sigmatau2 as usize
            + self.v3lapl3 as usize
            + self.v3lapl2tau as usize
            + self.v3lapltau2 as usize
            + self.v3tau3 as usize;
        let o4 = self.v4rho4 as usize
            + self.v4rho3sigma as usize
            + self.v4rho3lapl as usize
            + self.v4rho3tau as usize
            + self.v4rho2sigma2 as usize
            + self.v4rho2sigmalapl as usize
            + self.v4rho2sigmatau as usize
            + self.v4rho2lapl2 as usize
            + self.v4rho2lapltau as usize
            + self.v4rho2tau2 as usize
            + self.v4rhosigma3 as usize
            + self.v4rhosigma2lapl as usize
            + self.v4rhosigma2tau as usize
            + self.v4rhosigmalapl2 as usize
            + self.v4rhosigmalapltau as usize
            + self.v4rhosigmatau2 as usize
            + self.v4rholapl3 as usize
            + self.v4rholapl2tau as usize
            + self.v4rholapltau2 as usize
            + self.v4rhotau3 as usize
            + self.v4sigma4 as usize
            + self.v4sigma3lapl as usize
            + self.v4sigma3tau as usize
            + self.v4sigma2lapl2 as usize
            + self.v4sigma2lapltau as usize
            + self.v4sigma2tau2 as usize
            + self.v4sigmalapl3 as usize
            + self.v4sigmalapl2tau as usize
            + self.v4sigmalapltau2 as usize
            + self.v4sigmatau3 as usize
            + self.v4lapl4 as usize
            + self.v4lapl3tau as usize
            + self.v4lapl2tau2 as usize
            + self.v4lapltau3 as usize
            + self.v4tau4 as usize;
        o0 + o1 + o2 + o3 + o4
    }

    fn zeroed() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_unpolarized() {
        let d = Dimensions::lda(Spin::Unpolarized);
        assert_eq!(d.rho, 1);
        assert_eq!(d.vrho, 1);
        assert_eq!(d.v2rho2, 1);
        assert_eq!(d.sigma, 0);
        assert_eq!(d.lapl, 0);
        assert_eq!(d.tau, 0);
    }

    #[test]
    fn test_lda_polarized() {
        let d = Dimensions::lda(Spin::Polarized);
        assert_eq!(d.rho, 2);
        assert_eq!(d.vrho, 2);
        assert_eq!(d.v2rho2, 3);
        assert_eq!(d.v3rho3, 4);
        assert_eq!(d.v4rho4, 5);
    }

    #[test]
    fn test_gga_unpolarized() {
        let d = Dimensions::gga(Spin::Unpolarized);
        assert_eq!(d.sigma, 1);
        assert_eq!(d.vsigma, 1);
        assert_eq!(d.v2rhosigma, 1);
    }

    #[test]
    fn test_gga_polarized() {
        let d = Dimensions::gga(Spin::Polarized);
        assert_eq!(d.sigma, 3);
        assert_eq!(d.vsigma, 3);
        assert_eq!(d.v2rhosigma, 6);
        assert_eq!(d.v4sigma4, 15);
    }

    #[test]
    fn test_mgga_polarized_total() {
        let d = Dimensions::mgga(Spin::Polarized);
        // Total output components for polarized MGGA across all orders.
        // Verified against libxc util.c dimension values.
        assert_eq!(d.total_output_components(), 767);
    }

    #[test]
    fn test_lda_no_sigma_lapl_tau() {
        let d = Dimensions::lda(Spin::Polarized);
        assert_eq!(d.sigma, 0);
        assert_eq!(d.lapl, 0);
        assert_eq!(d.tau, 0);
        assert_eq!(d.vsigma, 0);
        assert_eq!(d.vlapl, 0);
        assert_eq!(d.vtau, 0);
        assert_eq!(d.v2rhosigma, 0);
        assert_eq!(d.v2sigmalapl, 0);
    }

    #[test]
    fn test_mgga_polarized_order4_spot_checks() {
        let d = Dimensions::mgga(Spin::Polarized);
        // Spot-check specific order-4 values from libxc util.c
        assert_eq!(d.v4rho3lapl, 8);       // 4*2
        assert_eq!(d.v4rhosigma2lapl, 36);  // 3*6*2
        assert_eq!(d.v4sigma3tau, 30);      // 10*3
        assert_eq!(d.v4lapl4, 5);
        assert_eq!(d.v4tau4, 5);
    }

    #[test]
    fn test_mgga_unpolarized_all_ones() {
        let d = Dimensions::mgga(Spin::Unpolarized);
        // For unpolarized MGGA, all derivative dimensions are 1
        assert_eq!(d.v2rholapl, 1);
        assert_eq!(d.v3sigmalapltau, 1);
        assert_eq!(d.v4sigmalapltau2, 1);
        assert_eq!(d.v4tau4, 1);
    }
}
