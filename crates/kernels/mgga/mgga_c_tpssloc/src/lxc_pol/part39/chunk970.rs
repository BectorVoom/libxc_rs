//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 970/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk970<F: Float>(t13191: F, t2701: F, t820: F, t1484: F, t2553: F, t2563: F, t4159: F, t119: F, t12971: F, t210: F, t4155: F, t9573: F, t2645: F, t2684: F, t4248: F, t13076: F, t13080: F, t13084: F, t13087: F, t13173: F, t13177: F, t13182: F, t13186: F, t13190: F, t2623: F, t2643: F, t2681: F, t4167: F, t4178: F, t4257: F, t787: F, t817: F, t831: F, t843: F, t9602: F, t9604: F) -> (F,) {
    let t13193 = t2701 * t820 * t13191;
    let t13196 = t1484 * t2553;
    let t13198 = t2701 * t820 * t13196;
    let t13202 = 7.0 / 72.0 * t2563 * t4159;
    let t13203 = t119 * t12971;
    let t13204 = t210 * t13203;
    let t13208 = 7.0 / 24.0 * t9573 * t4155;
    let t13210 = t2645 * t4248 * t2684;
    let t13213 = -t2643 * t13076 / 3072.0 - 5.0 / 768.0 * t2643 * t13080 - t4178 * t13084 / 384.0 - 35.0 / 216.0 * t13087 - 119.0 / 1728.0 * t9602 + 7.0 / 1152.0 * t9604 + 5.0 / 384.0 * t2623 * t4257 - t817 * t13173 / 3072.0 - t13177 * t831 / 1536.0 - t4167 * t2681 / 3072.0 - 119.0 / 13824.0 * t13182 - 5.0 / 128.0 * t843 * t13186 - t13190 + 5.0 / 384.0 * t843 * t13193 + 5.0 / 768.0 * t843 * t13198 + t13202 - t787 * t13204 / 48.0 - t13208 + t2643 * t13210 / 768.0;
    (t13213,)
}
