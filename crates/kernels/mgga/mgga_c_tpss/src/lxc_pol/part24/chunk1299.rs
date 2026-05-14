//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1299/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1299<F: Float>(t1265: F, t5381: F, t12828: F, t4459: F, t21053: F, t219: F, t43101: F, t520: F, t5413: F, t1640: F, t43602: F, t5408: F, t1266: F, t13880: F, t13940: F, t1768: F, t18474: F, t18496: F, t18497: F, t19500: F, t19509: F, t19521: F, t19531: F, t19535: F, t19536: F, t19540: F, t19541: F, t19554: F, t19564: F, t21052: F, t4517: F, t5449: F, t5734: F, t5739: F, t5740: F, t60653: F, t65706: F, t65788: F, t65871: F) -> (F,) {
    let t69704 = t5381 * t1265;
    let t69708 = t12828 * t4459;
    let t69714 = t21053 * t219;
    let t69727 = t43101 * t520;
    let t69730 = t5413 * t1265;
    let t69734 = t1640 * t4459;
    let t69738 = t43602 * t520;
    let t69741 = t5408 * t1265;
    let t69763 = 4.0 * t18496 * t19541 * t69704 - 4.0 * t19540 * t19541 * t69708 - 6.0 * t5734 * t13880 - t69714 * t1266 - 4.0 * t65871 * t19536 + 12.0 * t60653 * t65788 * t19535 + 2.0 * t19509 * t19564 - t18474 * t5449 - 4.0 * t18496 * t65706 * t19521 + t19540 * t18497 * t69727 - 2.0 * t18496 * t18497 * t69730 + 2.0 * t19540 * t18497 * t69734 + t19540 * t18497 * t69738 - 2.0 * t18496 * t18497 * t69741 - 4.0 * t18496 * t65706 * t19535 + 2.0 * t5739 * t5740 * t21052 * t1265 - 2.0 * t19500 * t4517 - 4.0 * t18496 * t65788 * t19554 + 2.0 * t5739 * t5740 * t1768 * t13940 + 4.0 * t19509 * t19531;
    (t69763,)
}
