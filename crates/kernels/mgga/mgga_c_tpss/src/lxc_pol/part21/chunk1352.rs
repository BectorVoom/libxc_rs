//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1352/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1352<F: Float>(t1265: F, t4460: F, t18495: F, t6259: F, t10085: F, t1768: F, t3259: F, t41590: F, t12828: F, t13109: F, t1657: F, t18483: F, t18496: F, t18497: F, t18500: F, t18514: F, t18519: F, t18524: F, t18527: F, t19500: F, t19509: F, t19531: F, t19535: F, t19540: F, t19541: F, t19542: F, t19548: F, t19552: F, t19554: F, t19555: F, t19559: F, t19564: F, t3255: F, t3367: F, t5731: F, t5734: F, t60659: F, t60847: F, t65719: F) -> (F,) {
    let t65867 = t4460 * t1265;
    let t65871 = t6259 * t18495;
    let t65877 = t10085 * t1768;
    let t65878 = t41590 * t3259;
    let t65882 = t12828 * t3259;
    let t65892 = 2.0 * t18483 * t19548 + 2.0 * t18483 * t19552 + 2.0 * t19509 * t18519 + t19509 * t18524 + 2.0 * t19500 * t3367 - t5734 * t13109 + 2.0 * t18483 * t19564 + t19509 * t18527 - 2.0 * t19509 * t18514 + 4.0 * t18483 * t19531 - t60847 * t1657 + 2.0 * t65719 * t19555 - 4.0 * t18496 * t60659 * t19535 - 4.0 * t18496 * t18497 * t65867 - 4.0 * t65871 * t18500 + 2.0 * t19540 * t60659 * t19554 + 6.0 * t19540 * t65877 * t65878 - 6.0 * t19540 * t19541 * t65882 - 4.0 * t19540 * t3255 * t5731 * t19542 + 4.0 * t18483 * t19559;
    (t65892,)
}
