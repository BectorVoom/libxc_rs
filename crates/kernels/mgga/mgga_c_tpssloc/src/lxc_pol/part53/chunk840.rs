//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 840/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk840<F: Float>(t1307: F, t22690: F, t22792: F, t6950: F, t1332: F, t31175: F, t8467: F, t2690: F, t544: F, t553: F, t1351: F, t22705: F, t22852: F, t550: F, t59: F, t31338: F, t81651: F, t82074: F) -> (F, F, F, F, F) {
    let t114031 = t22792 * t22690 * t6950 * t1307;
    let t114034 = t1332 * t31175 * t8467;
    let t114038 = t544 * t553 * t2690 * t8467;
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114592 = t81651 * t82074 * t31338;
    (t114031, t114034, t114038, t114046, t114592)
}
