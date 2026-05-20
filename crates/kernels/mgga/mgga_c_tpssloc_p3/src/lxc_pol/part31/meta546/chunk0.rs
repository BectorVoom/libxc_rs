//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1771/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1771<F: Float>(t23145: F, t2617: F, t23102: F, t80782: F, t23113: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t22813: F, t6589: F) -> (F, F, F, F, F, F, F) {
    let t81865 = t2617 * t23145;
    let t81876 = t23102 * t80782;
    let t81877 = t81876 * t23113;
    let t81882 = t23093 * t281;
    let t81883 = t81882 * t23113;
    let t81886 = t812 * t23046 * t835;
    let t81902 = t22813 * t6589 * t80782;
    (t81865, t81876, t81877, t81882, t81883, t81886, t81902)
}
