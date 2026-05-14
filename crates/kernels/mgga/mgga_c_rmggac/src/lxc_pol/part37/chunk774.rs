//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 774/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk774<F: Float>(t69421: F, t74974: F, t69171: F, t74978: F, t12108: F, t69511: F, t12111: F, t69176: F, t12117: F, t69439: F, t69484: F, t75953: F, t75957: F, t69428: F, t75963: F, t74960: F, t7788: F) -> (F, F, F, F, F, F, F, F, F) {
    let t76169 = t69421 * t74974;
    let t76171 = t69171 * t74978;
    let t76173 = t69511 * t12108;
    let t76175 = t69176 * t12111;
    let t76178 = t69439 * t12117;
    let t76180 = t69484 * t75953;
    let t76182 = t69171 * t75957;
    let t76184 = t69428 * t75963;
    let t76186 = t7788 * t74960;
    (t76169, t76171, t76173, t76175, t76178, t76180, t76182, t76184, t76186)
}
