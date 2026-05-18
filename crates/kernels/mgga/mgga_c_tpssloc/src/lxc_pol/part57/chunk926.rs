//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 926/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk926<F: Float>(t32826: F, t6562: F, t794: F, t22893: F, t23164: F, t32818: F, t32827: F, t6547: F, t23168: F, t32819: F, t234: F, t7510: F) -> (F, F, F, F, F) {
    let t118709 = t6562 * t794 * t32826;
    let t118727 = t23164 * t22893 * t32818;
    let t118738 = t6547 * t32827;
    let t118744 = t23168 * t32819;
    let t118747 = t234 * t7510;
    (t118709, t118727, t118738, t118744, t118747)
}
