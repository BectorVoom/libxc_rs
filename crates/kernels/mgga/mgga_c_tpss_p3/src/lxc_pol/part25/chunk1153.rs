//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1153/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1153<F: Float>(t1114: F, t15917: F, t3931: F, t4232: F, t1125: F, t12550: F, t15882: F, t15886: F, t15891: F, t15895: F, t15899: F, t15902: F, t15906: F, t15910: F, t15914: F, t3052: F, t3067: F, t3080: F, t9556: F, t9618: F) -> F {
    let t15918 = t15917 * t1114;
    let t15919 = t3931 * t15918;
    let t15923 = t15917 * t4232;
    let t15924 = t3931 * t15923;
    let t15927 = F::new(5.0) / F::new(13824.0) * t1125 * t15882 - F::new(5.0) / F::new(5184.0) * t1125 * t15886 + t9618 * t15891 / F::new(512.0) - t3067 * t15895 / F::new(2304.0) - t15899 / F::new(3456.0) - t3067 * t15902 / F::new(4608.0) - t9556 * t15906 / F::new(2304.0) - t3067 * t15910 / F::new(2304.0) + F::new(5.0) / F::new(13824.0) * t3067 * t15914 - t3080 * t15919 / F::new(3072.0) + t12550 / F::new(81.0) + t3052 * t15924 / F::new(1536.0);
    t15927
}
