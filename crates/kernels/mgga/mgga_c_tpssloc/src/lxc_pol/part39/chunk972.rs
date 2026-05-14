//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 972/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk972<F: Float>(t2642: F, t4166: F, t2617: F, t4177: F, t2628: F, t836: F, t812: F, t4184: F, t242: F, t9972: F, t2631: F, t9975: F, t4180: F, t4181: F, t13225: F, t13231: F, t13234: F, t13237: F, t13244: F, t13248: F, t2643: F, t2649: F, t4178: F, t4191: F, t4240: F, t9639: F, t9642: F, t9668: F, t9672: F, t9675: F, t9679: F, t9986: F, t9988: F, t9994: F) -> (F, F) {
    let t13251 = t4166 * t2642;
    let t13254 = t2617 * t4177;
    let t13257 = t2628 * t836;
    let t13258 = t812 * t13257;
    let t13260 = 7.0 / 1152.0 * t13258 * t4184;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13263 = t9975 * t2631;
    let t13265 = t4180 * t4181 * t13263;
    let t13268 = -7.0 / 576.0 * t9639 - 7.0 / 2304.0 * t9668 - 119.0 / 6912.0 * t9672 + 7.0 / 2304.0 * t9675 + 7.0 / 4608.0 * t9679 + 7.0 / 4608.0 * t9986 - 35.0 / 1152.0 * t9988 + 7.0 / 576.0 * t9994 + t2643 * t13225 / 384.0 - t4178 * t13231 / 192.0 + 119.0 / 13824.0 * t13234 - t13237 + t9642 * t4191 / 384.0 - t9642 * t4240 / 1536.0 + t4178 * t13244 / 768.0 + t4178 * t13248 / 1536.0 + t13251 * t2649 / 384.0 + t13254 * t4184 / 768.0 - t13260 - t13262 * t13265 / 512.0;
    (t13263, t13268)
}
