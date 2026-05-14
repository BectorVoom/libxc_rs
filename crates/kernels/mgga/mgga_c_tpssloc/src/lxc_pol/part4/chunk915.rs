//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 915/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk915<F: Float>(t16816: F, t16839: F, t4180: F, t4182: F, t5593: F, t9638: F, t5527: F, t776: F, t820: F, t9607: F, t16753: F, t819: F, t13087: F, t13182: F, t13190: F, t13202: F, t13208: F, t13234: F, t13237: F, t13262: F, t16836: F, t2618: F, t4172: F, t4178: F, t4184: F, t4257: F, t5587: F, t5614: F, t5619: F, t817: F, t843: F, t9602: F, t9672: F, t9967: F) -> (F,) {
    let t16841 = t4180 * t16839 * t16816;
    let t16845 = t4180 * t16839 * t4182;
    let t16848 = t9638 * t5593;
    let t16851 = t5527 * t776;
    let t16853 = t9607 * t820 * t16851;
    let t16859 = t819 * t820 * t16753;
    let t16869 = -35.0 / 108.0 * t13087 - 119.0 / 3456.0 * t9602 - 119.0 / 6912.0 * t13182 - t13190 + t13202 - t13208 + t16836 * t4184 / 768.0 - t13262 * t16841 / 512.0 + t4178 * t16845 / 512.0 - 7.0 / 576.0 * t16848 - 119.0 / 13824.0 * t9672 - 5.0 / 128.0 * t843 * t16853 - t2618 * t5614 / 3072.0 - t817 * t16859 / 3072.0 - t2618 * t5619 / 3072.0 + 5.0 / 384.0 * t4172 * t4257 + 119.0 / 6912.0 * t13234 - t13237 + t9967 * t5587 / 1536.0;
    (t16869,)
}
