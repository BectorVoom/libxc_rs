//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1349/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1349<F: Float>(t1219: F, t6255: F, t1265: F, t13047: F, t13108: F, t1768: F, t1775: F, t18474: F, t18481: F, t18483: F, t18496: F, t18497: F, t18499: F, t18530: F, t19497: F, t19517: F, t19521: F, t19527: F, t19540: F, t19541: F, t3326: F, t3384: F, t43908: F, t4494: F, t520: F, t538: F, t5734: F, t5739: F, t5740: F, t5742: F, t5745: F, t60659: F, t6260: F, t6271: F, t65654: F, t65667: F, t65685: F, t65691: F, t65696: F, t65703: F) -> (F,) {
    let t65706 = t1219 * t6255;
    let t65710 = 4.0 * t18483 * t19527 + param_beta * t65654 * t538 + 4.0 * t5739 * t5740 * t19497 * t1265 - 6.0 * t5734 * t13047 + 2.0 * t5739 * t5740 * t1768 * t13108 + 4.0 * t65667 * t5742 + t5739 * t5745 * t6255 * t3326 * t520 - 12.0 * t18483 * t19517 - t6260 * t18530 + 4.0 * t18474 * t4494 + 2.0 * t5739 * t5740 * t6255 * t3384 - t18481 * t6271 - t65685 * t1775 - 4.0 * t18496 * t60659 * t19521 - 4.0 * t18496 * t18497 * t65691 + 4.0 * t18496 * t19541 * t65696 + 2.0 * t19540 * t18497 * t43908 + t19540 * t18497 * t65703 - 4.0 * t18496 * t65706 * t18499;
    (t65710,)
}
