//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 930/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk930<F: Float>(t2696: F, t4166: F, t849: F, t13176: F, t842: F, t1516: F, t9601: F, t10012: F, t10014: F, t10026: F, t10029: F, t10030: F, t10036: F, t10038: F, t13333: F, t13337: F, t13345: F, t13347: F, t13353: F, t13359: F, t249: F, t2623: F, t2643: F, t2703: F, t2707: F, t4172: F, t4178: F, t4261: F, t843: F, t9990: F) -> (F,) {
    let t13360 = t4166 * t2696;
    let t13362 = 7.0 / 576.0 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    let t13375 = t4178 * t13333 / 512.0 + t13337 * t249 / 3072.0 - t9990 * t1516 / 768.0 - t2623 * t4261 / 384.0 + t13345 - t843 * t13347 / 768.0 - 5.0 / 384.0 * t2643 * t13353 - 7.0 / 4608.0 * t10012 + 119.0 / 6912.0 * t10014 - t10026 - t10029 + t13359 + t13362 - t4172 * t2707 / 768.0 - t13365 * t849 / 384.0 - 119.0 / 3456.0 * t13368 + 5.0 / 768.0 * t4172 * t2703 - 7.0 / 48.0 * t10030 - 35.0 / 108.0 * t10036 + 7.0 / 144.0 * t10038;
    (t13375,)
}
