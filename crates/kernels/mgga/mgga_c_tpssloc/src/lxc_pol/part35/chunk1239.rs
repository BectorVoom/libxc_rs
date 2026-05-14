//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1239/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1239<F: Float>(t1530: F, t5397: F, t21066: F, t25: F, t20800: F, t20947: F, t25014: F, t1408: F, t5527: F, t5664: F, t5660: F, t20778: F, t1877: F, t1915: F, t23295: F, t25013: F, t2522: F, t25358: F, t28252: F, t28256: F, t28448: F, t28456: F, t28459: F, t28462: F, t4314: F, t6670: F, t7475: F, t7541: F, t82312: F, t87975: F) -> (F,) {
    let t105780 = t5397 * t1530;
    let t105787 = t25 * t21066;
    let t105797 = t25 * t20800;
    let t105801 = t25014 * t20947;
    let t105810 = t1408 * t5527;
    let t105814 = t1408 * t5664;
    let t105818 = t1408 * t5660;
    let t105822 = t25 * t20778;
    let t105829 = -3.0 / 2.0 * t1877 * t25358 * t28462 - 3.0 / 2.0 * t1877 * t6670 * t105780 + 9.0 / 2.0 * t2522 * t28448 * t7475 - t1877 * t6670 * t105787 / 2.0 + 3.0 * t1877 * t87975 * t28456 + 9.0 * t2522 * t7541 * t28252 + 3.0 / 2.0 * t2522 * t1915 * t105797 + 9.0 * t25013 * t105801 - 3.0 * t1877 * t25358 * t28459 + 9.0 / 2.0 * t2522 * t7541 * t28256 + 9.0 * t4314 * t1915 * t105810 + 3.0 * t1877 * t23295 * t105814 - 3.0 / 2.0 * t1877 * t6670 * t105818 - 3.0 * t1877 * t82312 * t105822 + 3.0 / 2.0 * t1877 * t28448 * t1408;
    (t105829,)
}
