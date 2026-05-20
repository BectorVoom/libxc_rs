//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 519/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk519<F: Float>(t3032: F, t3499: F, t1208: F, t476: F, t478: F, t3036: F, t483: F, t475: F, t1210: F, t121: F, t1229: F, t1090: F, t248: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3500 = t3499 * t3032;
    let t3502 = F::new(1.0) / t1208 / t476;
    let t3503 = t3502 * t478;
    let t3504 = t483 * t3036;
    let t3505 = t3503 * t3504;
    let t3506 = t3500 * t3505;
    let t3508 = t475 * t475;
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3521 = t121 * t1229;
    let t3523 = t248 * t3521 * t1090;
    (t3500, t3502, t3503, t3504, t3506, t3508, t3515, t3521, t3523)
}
