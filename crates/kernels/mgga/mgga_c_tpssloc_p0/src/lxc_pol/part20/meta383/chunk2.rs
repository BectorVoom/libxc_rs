//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1748/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1748<F: Float>(t13263: F, t4180: F, t4181: F, t13225: F, t13231: F, t13234: F, t13237: F, t13244: F, t13248: F, t13251: F, t13254: F, t13260: F, t13262: F, t2643: F, t2649: F, t4178: F, t4184: F, t4191: F, t4240: F, t9639: F, t9642: F, t9668: F, t9672: F, t9675: F, t9679: F, t9986: F, t9988: F, t9994: F) -> (F, F) {
    let t13265 = t4180 * t4181 * t13263;
    let t13268 = -F::new(7.0) / F::new(576.0) * t9639 - F::new(7.0) / F::new(2304.0) * t9668 - F::new(119.0) / F::new(6912.0) * t9672 + F::new(7.0) / F::new(2304.0) * t9675 + F::new(7.0) / F::new(4608.0) * t9679 + F::new(7.0) / F::new(4608.0) * t9986 - F::new(35.0) / F::new(1152.0) * t9988 + F::new(7.0) / F::new(576.0) * t9994 + t2643 * t13225 / F::new(384.0) - t4178 * t13231 / F::new(192.0) + F::new(119.0) / F::new(13824.0) * t13234 - t13237 + t9642 * t4191 / F::new(384.0) - t9642 * t4240 / F::new(1536.0) + t4178 * t13244 / F::new(768.0) + t4178 * t13248 / F::new(1536.0) + t13251 * t2649 / F::new(384.0) + t13254 * t4184 / F::new(768.0) - t13260 - t13262 * t13265 / F::new(512.0);
    (t13265, t13268)
}
