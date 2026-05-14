//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 988/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk988<F: Float>(t23102: F, t80782: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t22813: F, t6589: F, t23138: F, t6604: F, t22690: F, t2627: F, t10024: F, t1899: F, t213: F, t9223: F) -> (F, F, F, F, F, F, F, F) {
    let t81876 = t23102 * t80782;
    let t81882 = t23093 * t281;
    let t81886 = t812 * t23046 * t835;
    let t81902 = t22813 * t6589 * t80782;
    let t81911 = t23138 * t6604;
    let t81914 = t22690 * t2627;
    let t81920 = t1899 * t10024;
    let t81933 = t9223 * t6589 * t213;
    (t81876, t81882, t81886, t81902, t81911, t81914, t81920, t81933)
}
