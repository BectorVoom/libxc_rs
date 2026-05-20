//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1476/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1476<F: Float>(t13076: F, t13080: F, t13084: F, t13087: F, t13173: F, t13177: F, t13182: F, t13186: F, t13190: F, t13193: F, t13198: F, t13202: F, t13204: F, t13208: F, t13210: F, t2623: F, t2643: F, t2681: F, t4167: F, t4178: F, t4257: F, t787: F, t817: F, t831: F, t843: F, t9602: F, t9604: F) -> F {
    let t13213 = -t2643 * t13076 / F::new(3072.0) - F::new(5.0) / F::new(768.0) * t2643 * t13080 - t4178 * t13084 / F::new(384.0) - F::new(35.0) / F::new(216.0) * t13087 - F::new(119.0) / F::new(1728.0) * t9602 + F::new(7.0) / F::new(1152.0) * t9604 + F::new(5.0) / F::new(384.0) * t2623 * t4257 - t817 * t13173 / F::new(3072.0) - t13177 * t831 / F::new(1536.0) - t4167 * t2681 / F::new(3072.0) - F::new(119.0) / F::new(13824.0) * t13182 - F::new(5.0) / F::new(128.0) * t843 * t13186 - t13190 + F::new(5.0) / F::new(384.0) * t843 * t13193 + F::new(5.0) / F::new(768.0) * t843 * t13198 + t13202 - t787 * t13204 / F::new(48.0) - t13208 + t2643 * t13210 / F::new(768.0);
    t13213
}
