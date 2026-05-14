//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 889/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk889<F: Float>(t3034: F, t371: F, t1930: F, t1940: F, t3046: F, t354: F, t3053: F, t6765: F, t3127: F, t3037: F, t3033: F, t6753: F, t1004: F, t6764: F, t1036: F, t6750: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t23508 = 1.0 / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23528 = t1940 * t3046;
    let t23529 = t354 * t23528;
    let t23533 = t6765 * t3053;
    let t23535 = t3127 * sigma0;
    let t23536 = t23535 * t3037;
    let t23537 = t3033 * t23536;
    let t23540 = t6753 * t3037;
    let t23541 = t3033 * t23540;
    let t23544 = t1004 * t6764;
    let t23554 = t6750 * t1036;
    (t23509, t23529, t23533, t23537, t23541, t23544, t23554)
}
