//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1062/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1062<F: Float>(t14911: F, t4283: F, t3931: F, t4212: F, t4216: F, t140: F, t5206: F, t1098: F, t3054: F, t4245: F, t4231: F, t4246: F, t461: F, t5248: F, t4232: F, t1015: F, t5254: F) -> (F, F, F, F, F, F, F, F) {
    let t15827 = t4283 * t14911;
    let t15828 = t3931 * t15827;
    let t15832 = t4212 * t4216;
    let t15834 = t140 * t5206;
    let t15835 = t1098 * t15834;
    let t15837 = t3054 * t4245;
    let t15838 = t4231 * t15837;
    let t15839 = t3931 * t15838;
    let t15842 = t4231 * t4246;
    let t15843 = t3931 * t15842;
    let t15846 = t461 * t5248;
    let t15847 = t15846 * t4232;
    let t15848 = t3931 * t15847;
    let t15854 = t5254 * t1015;
    (t15828, t15832, t15835, t15839, t15843, t15846, t15848, t15854)
}
