//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 531/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk531<F: Float>(t14290: F, t556: F, t14293: F, t2842: F, t27: F, t29: F, t570: F, t14296: F, t14302: F, t15106: F, t14305: F, t15109: F, t3046: F, t1326: F, t14309: F, t15087: F, t262: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15132 = t14290 * t556;
    let t15134 = t14293 * t2842;
    let t15137 = t27 * t29 * t570;
    let t15138 = t14296 * t15137;
    let t15140 = t14302 * t15106;
    let t15142 = t14305 * t15109;
    let t15144 = t3046 * t570;
    let t15146 = t14309 * t1326 * t15144;
    let t15163 = t262 * t15087;
    (t15132, t15134, t15137, t15138, t15140, t15142, t15144, t15146, t15163)
}
