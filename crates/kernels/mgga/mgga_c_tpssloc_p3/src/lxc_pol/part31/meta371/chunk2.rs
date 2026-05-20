//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1314/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1314<F: Float>(t13087: F, t13182: F, t13190: F, t13202: F, t13208: F, t13234: F, t13237: F, t13262: F, t16836: F, t16841: F, t16845: F, t16848: F, t16853: F, t16859: F, t2618: F, t4172: F, t4178: F, t4184: F, t4257: F, t5587: F, t5614: F, t5619: F, t817: F, t843: F, t9602: F, t9672: F, t9967: F) -> F {
    let t16869 = -F::new(35.0) / F::new(108.0) * t13087 - F::new(119.0) / F::new(3456.0) * t9602 - F::new(119.0) / F::new(6912.0) * t13182 - t13190 + t13202 - t13208 + t16836 * t4184 / F::new(768.0) - t13262 * t16841 / F::new(512.0) + t4178 * t16845 / F::new(512.0) - F::new(7.0) / F::new(576.0) * t16848 - F::new(119.0) / F::new(13824.0) * t9672 - F::new(5.0) / F::new(128.0) * t843 * t16853 - t2618 * t5614 / F::new(3072.0) - t817 * t16859 / F::new(3072.0) - t2618 * t5619 / F::new(3072.0) + F::new(5.0) / F::new(384.0) * t4172 * t4257 + F::new(119.0) / F::new(6912.0) * t13234 - t13237 + t9967 * t5587 / F::new(1536.0);
    t16869
}
