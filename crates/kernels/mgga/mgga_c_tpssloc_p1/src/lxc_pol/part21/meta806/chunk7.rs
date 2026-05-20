//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2805/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2805<F: Float>(t5619: F, t9671: F, t13222: F, t13229: F, t13352: F, t16976: F, t20981: F, t2701: F, t2703: F, t4178: F, t4281: F, t4291: F, t47269: F, t47271: F, t47273: F, t47276: F, t47279: F, t47283: F, t5585: F, t58090: F, t59251: F, t59255: F, t59257: F, t59259: F, t59261: F, t59263: F, t59265: F, t59267: F, t820: F, t843: F) -> F {
    let t59276 = t9671 * t5619;
    let t59278 = -F::new(119.0) / F::new(864.0) * t47269 + F::new(7.0) / F::new(576.0) * t47271 + F::new(7.0) / F::new(288.0) * t47273 - F::new(119.0) / F::new(864.0) * t47276 + F::new(7.0) / F::new(288.0) * t47279 + F::new(7.0) / F::new(576.0) * t47283 - t4178 * t13222 * t5585 * t13229 / F::new(64.0) + F::new(5.0) / F::new(384.0) * t843 * t2701 * t820 * t58090 - F::new(35.0) / F::new(576.0) * t59251 + F::new(5.0) / F::new(768.0) * t16976 * t2703 + F::new(7.0) / F::new(576.0) * t59255 + F::new(7.0) / F::new(576.0) * t59257 - F::new(119.0) / F::new(1728.0) * t59259 + F::new(7.0) / F::new(288.0) * t59261 - F::new(119.0) / F::new(3456.0) * t59263 - F::new(5.0) / F::new(192.0) * t4291 * t59265 * t59267 * t13352 + F::new(5.0) / F::new(96.0) * t4281 * t59265 * t59267 * t20981 * t13229 - F::new(119.0) / F::new(13824.0) * t59276;
    t59278
}
