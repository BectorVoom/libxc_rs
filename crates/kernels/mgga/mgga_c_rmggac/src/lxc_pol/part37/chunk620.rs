//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 620/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk620<F: Float>(t1330: F, t637: F, t797: F, t3093: F, t35206: F, t2048: F, t6444: F, t2044: F, t25525: F, t25640: F, t3068: F, t854: F, t3102: F, t3851: F, t69239: F, t25518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69270 = t797 * t1330 * t637;
    let t69272 = t3093 * t35206;
    let t69274 = t6444 * t2048;
    let t69276 = t25525 * t2044;
    let t69279 = t25640 * t3068;
    let t69287 = t854 * t1330 * t637;
    let t69289 = t3102 * t35206;
    let t69294 = t3851 * t69239;
    let t69296 = t25518 * t3068;
    (t69270, t69272, t69274, t69276, t69279, t69287, t69289, t69294, t69296)
}
