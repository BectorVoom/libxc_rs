//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1301/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1301<F: Float>(t19539: F, t6259: F, t1232: F, t43710: F, t5381: F, t1656: F, t4459: F, t520: F, t5432: F, t1639: F, t4516: F, t5448: F, t1265: F, t18474: F, t18483: F, t18490: F, t18496: F, t18497: F, t18511: F, t19522: F, t19540: F, t19541: F, t19542: F, t19543: F, t19554: F, t19555: F, t21074: F, t21075: F, t21078: F, t21079: F, t21082: F, t3255: F, t3260: F, t51545: F, t5407: F, t5433: F, t5731: F, t5739: F, t5745: F, t60649: F, t60653: F, t60659: F, t6255: F, t65706: F, t65871: F, t65877: F, t69587: F) -> (F,) {
    let t69654 = t6259 * t19539;
    let t69663 = t43710 * t1232;
    let t69667 = t5381 * t1232;
    let t69676 = t1656 * t4459 * t520;
    let t69681 = t5432 * t1232 * t520;
    let t69691 = t4516 * t1639 * t520;
    let t69699 = t5448 * t1232 * t520;
    let t69703 = -2.0 * t5739 * t18511 * t69587 * t3260 + 4.0 * t18483 * t21079 - 12.0 * t5739 * t18490 * t21078 * t1265 - 2.0 * t19540 * t19541 * t51545 - 4.0 * t65871 * t19522 + 2.0 * t18474 * t5433 + t5739 * t5745 * t5731 * t5407 * t520 + 2.0 * t69654 * t19555 - 4.0 * t69654 * t19543 - 6.0 * t5739 * t18490 * t21082 * t1265 + 6.0 * t19540 * t65877 * t69663 - 6.0 * t19540 * t19541 * t69667 - 4.0 * t19540 * t3255 * t6255 * t19542 - 4.0 * t18496 * t18497 * t69676 + 6.0 * t60653 * t18497 * t69681 - 4.0 * t60649 * t21075 - 4.0 * t18496 * t60659 * t21074 - 4.0 * t18496 * t18497 * t69691 + 2.0 * t19540 * t65706 * t19554 - 2.0 * t18496 * t18497 * t69699;
    (t69703,)
}
