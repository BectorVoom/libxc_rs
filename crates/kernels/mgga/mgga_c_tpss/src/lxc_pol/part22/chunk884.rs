//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 884/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk884<F: Float>(t7821: F, t7824: F, t7827: F, t7830: F, t7834: F, t7836: F, t7838: F, t7841: F, t705: F, t2271: F, t697: F, t164: F) -> (F, F) {
    let t7843 = -F::new(0.47063e1) * t7821 + F::cast_from(0.31375333333333333334e1_f64) * t7824 - F::cast_from(0.36604555555555555556e1_f64) * t7827 - F::cast_from(0.16068111111111111111e1_f64) * t7830 + F::cast_from(0.28051666666666666666e0_f64) * t7834 - F::cast_from(0.56103333333333333332e0_f64) * t7836 - F::cast_from(0.6545388888888888889e0_f64) * t7838 - F::cast_from(0.46308888888888888888e0_f64) * t7841;
    let t7844 = t7843 * t705;
    let t7848 = F::new(1.0) / t2271 / t697;
    let t7849 = t164 * t7848;
    (t7844, t7849)
}
