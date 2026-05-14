//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1116/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1116<F: Float>(t15901: F, t3068: F, t1015: F, t5249: F, t1562: F, t4056: F, t1114: F, t5064: F, t9702: F, t461: F, t5242: F, t3931: F, t4232: F, t1125: F, t12550: F, t15882: F, t15886: F, t15891: F, t15895: F, t15899: F, t3052: F, t3067: F, t3080: F, t9556: F, t9618: F) -> (F,) {
    let t15902 = t3068 * t15901;
    let t15905 = t5249 * t1015;
    let t15906 = t3068 * t15905;
    let t15909 = t1562 * t4056;
    let t15910 = t3068 * t15909;
    let t15913 = t5064 * t1114;
    let t15914 = t9702 * t15913;
    let t15917 = t461 * t5242;
    let t15918 = t15917 * t1114;
    let t15919 = t3931 * t15918;
    let t15923 = t15917 * t4232;
    let t15924 = t3931 * t15923;
    let t15927 = 5.0 / 13824.0 * t1125 * t15882 - 5.0 / 5184.0 * t1125 * t15886 + t9618 * t15891 / 512.0 - t3067 * t15895 / 2304.0 - t15899 / 3456.0 - t3067 * t15902 / 4608.0 - t9556 * t15906 / 2304.0 - t3067 * t15910 / 2304.0 + 5.0 / 13824.0 * t3067 * t15914 - t3080 * t15919 / 3072.0 + t12550 / 81.0 + t3052 * t15924 / 1536.0;
    (t15927,)
}
