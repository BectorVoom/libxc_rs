//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1276/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1276<F: Float>(t1332: F, t31175: F, t8467: F, t2690: F, t544: F, t553: F, t1351: F, t22705: F, t22852: F, t550: F, t59: F, t22751: F, t31195: F) -> (F, F, F, F) {
    let t114034 = t1332 * t31175 * t8467;
    let t114035 = F::new(7.0) / F::new(1152.0) * t114034;
    let t114038 = t544 * t553 * t2690 * t8467;
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114057 = t22751 * t31195;
    (t114035, t114038, t114046, t114057)
}
