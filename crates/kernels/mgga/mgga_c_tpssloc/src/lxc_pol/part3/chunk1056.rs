//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1056/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1056<F: Float>(t11575: F, t4904: F, t134: F, t3439: F, t461: F, t4724: F, t3447: F, t11514: F, t11556: F, t11558: F, t11561: F, t15391: F, t15396: F, t15401: F, t15405: F, t15406: F, t15409: F, t15412: F) -> (F,) {
    let t15415 = t11575 * t4904;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15420 = t15419 * t4724;
    let t15422 = 0.24691358024691358024e-3 * t3447 * t15420;
    let t15423 = -0.27777777777777777777e-3 * t11514 + 0.37037037037037037036e-3 * t11558 - 0.27777777777777777777e-3 * t11561 + t11556 - 0.37037037037037037036e-3 * t3447 * t15391 - 0.86419753086419753084e-3 * t3447 * t15396 + t15401 - t15405 + 0.74074074074074074072e-3 * t3447 * t15406 + 0.37037037037037037036e-3 * t3447 * t15409 + 0.22222222222222222221e-2 * t3447 * t15412 + 0.27777777777777777777e-3 * t3447 * t15415 + t15422;
    (t15423,)
}
