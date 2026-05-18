//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 894/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk894<F: Float>(t7821: F, t7824: F, t7827: F, t7830: F, t7834: F, t7836: F, t7838: F, t7841: F, t676: F, t657: F) -> F {
    let t7969 = -F::new(0.25319e1) * t7821 + F::new(0.16879333333333333333e1) * t7824 - F::new(0.19692555555555555555e1) * t7827 - F::new(0.93011851851851851854e0) * t7830 + F::new(0.13651666666666666667e0) * t7834 - F::new(0.27303333333333333333e0) * t7836 - F::new(0.3185388888888888889e0) * t7838 - F::new(0.36514074074074074075e0) * t7841;
    let t7970 = t7969 * t676;
    let t7972 = F::new(1.0) * t657 * t7970;
    t7972
}
