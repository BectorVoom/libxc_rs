//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1362/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1362<F: Float>(t21609: F, t219: F, t1395: F, t1396: F, t14363: F, t14367: F, t14424: F, t1707: F, t1708: F, t17993: F, t1809: F, t18753: F, t19734: F, t19736: F, t20446: F, t20449: F, t20471: F, t20483: F, t20488: F, t20492: F, t20506: F, t21631: F, t21640: F, t21656: F, t228: F, t3722: F, t4784: F, t5568: F, t5571: F, t5572: F, t5834: F, t5838: F, t6135: F, t6351: F, t66525: F, t69912: F, t70039: F, t70189: F, t72079: F, t819: F) -> F {
    let t72153 = t21609 * t219;
    let t72170 = -t5834 * t14424 + F::cast_from(4.0_f64) * t5834 * t14367 - F::cast_from(2.0_f64) * t20449 * t3722 - F::cast_from(2.0_f64) * t17993 * t21640 - F::cast_from(2.0_f64) * t6135 * t20506 - t70189 * t1809 + F::cast_from(2.0_f64) * t69912 * t5838 - F::cast_from(6.0_f64) * t5834 * t14363 - t5568 * t21656 - F::cast_from(2.0_f64) * t66525 * t1396 + F::cast_from(2.0_f64) * t18753 * t4784 - F::cast_from(4.0_f64) * t70039 * t20483 + F::cast_from(4.0_f64) * t17993 * t21631 - t72153 * t819 - t1707 * t1708 * t228 * t72079 + F::cast_from(2.0_f64) * t19736 * t20488 + F::cast_from(2.0_f64) * t19736 * t20492 - F::cast_from(2.0_f64) * t19734 * t6351 + F::cast_from(4.0_f64) * t19736 * t20471 + F::cast_from(4.0_f64) * t5571 * t5572 * t20446 * t1395;
    t72170
}
