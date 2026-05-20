//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 927/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk927<F: Float>(t23110: F, t23185: F, t32822: F, t2717: F, t7537: F, t112943: F, t6562: F, t7488: F, t32792: F, t6547: F, t23204: F, t32866: F) -> (F, F, F, F, F) {
    let t118766 = t23185 * t23110 * t32822;
    let t118821 = t2717 * t7537;
    let t118830 = t6562 * t112943 * t7488;
    let t118858 = t6547 * t32792;
    let t118885 = t6562 * t23204 * t32866;
    (t118766, t118821, t118830, t118858, t118885)
}
