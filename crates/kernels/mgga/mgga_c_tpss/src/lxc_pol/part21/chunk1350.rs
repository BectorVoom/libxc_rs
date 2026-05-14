//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1350/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1350<F: Float>(t520: F, t65695: F, t1640: F, t3384: F, t19539: F, t5736: F, t3366: F, t12828: F, t3326: F, t41371: F, t19498: F, t219: F, t3259: F, t6255: F, t1266: F, t12957: F, t13055: F, t1656: F, t1768: F, t1772: F, t1773: F, t18471: F, t18490: F, t18492: F, t18496: F, t18497: F, t18511: F, t19509: F, t19522: F, t19540: F, t19541: F, t19543: F, t19567: F, t3260: F, t43933: F, t4516: F, t522: F, t5731: F, t5734: F, t5737: F, t5739: F, t5740: F, t5745: F, t60649: F, t60653: F, t60811: F, t6262: F, t65654: F) -> (F, F) {
    let t65711 = t65695 * t520;
    let t65715 = t1640 * t3384;
    let t65719 = t5736 * t19539;
    let t65722 = t1640 * t3366;
    let t65729 = t12828 * t3326;
    let t65738 = t41371 * t520;
    let t65747 = t19498 * t219;
    let t65766 = t6255 * t3259;
    let t65778 = -2.0 * t18496 * t18497 * t65711 - 2.0 * t18496 * t18497 * t65715 - 4.0 * t65719 * t19543 + 6.0 * t60653 * t18497 * t65722 - 4.0 * t19540 * t19541 * t43933 - 2.0 * t19540 * t19541 * t65729 - t1772 * t1773 * t522 * t65654 - 4.0 * t60649 * t19522 + t19540 * t18497 * t65738 + 2.0 * t5734 * t13055 + t5739 * t5745 * t1768 * t12957 * t520 - 2.0 * t65747 * t1266 + 4.0 * t5739 * t5740 * t5731 * t4516 - 6.0 * t19509 * t18492 + 24.0 * t5739 * t60811 * t6262 * t3366 + 2.0 * t5739 * t5740 * t18471 * t1656 - 2.0 * t5737 * t19567 - 2.0 * t5739 * t18511 * t65766 * t3260 + t5739 * t5745 * t65766 * t520 - 6.0 * t5739 * t18490 * t6262 * t3384;
    (t65719, t65778)
}
