//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 836/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk836<F: Float>(t667: F, t7826: F, t124: F, t138: F, t599: F, t7821: F, t7824: F, t7827: F, t7830: F, t7834: F, t7836: F, t705: F, t2271: F, t697: F, t164: F, t2257: F, t704: F) -> (F, F, F, F, F) {
    let t7838 = t667 * t7826;
    let t7841 = t138 * t124 * t599;
    let t7843 = -0.47063e1 * t7821 + 0.31375333333333333334e1 * t7824 - 0.36604555555555555556e1 * t7827 - 0.16068111111111111111e1 * t7830 + 0.28051666666666666666e0 * t7834 - 0.56103333333333333332e0 * t7836 - 0.6545388888888888889e0 * t7838 - 0.46308888888888888888e0 * t7841;
    let t7844 = t7843 * t705;
    let t7848 = 1.0 / t2271 / t697;
    let t7849 = t164 * t7848;
    let t7850 = t2257 * t704;
    (t7838, t7841, t7844, t7849, t7850)
}
