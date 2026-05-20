//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1897/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1897<F: Float>(t26066: F, t72: F, t1410: F, t2235: F, t3961: F, t605: F, t3967: F, t1433: F, t645: F, t12725: F, t1873: F, t19456: F) -> (F, F, F, F, F, F, F) {
    let t26067 = t72 * t26066;
    let t26070 = t2235 * t1410;
    let t26073 = t605 * t3961;
    let t26076 = t605 * t3967;
    let t26090 = t72 * t1433 * t645;
    let t26109 = F::new(2.0) * t12725 * t1873;
    let t26111 = F::new(2.0) * t19456 * t1873;
    (t26067, t26070, t26073, t26076, t26090, t26109, t26111)
}
