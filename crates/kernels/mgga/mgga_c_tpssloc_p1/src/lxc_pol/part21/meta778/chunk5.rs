//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2695/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2695<F: Float>(t19635: F, t225: F, t20048: F, t1375: F, t1386: F, t16022: F, t16030: F, t16122: F, t16436: F, t16460: F, t16471: F, t16475: F, t1834: F, t1842: F, t19648: F, t20026: F, t3758: F, t3879: F, t3882: F, t3887: F, t3888: F, t3911: F, t40591: F, t5210: F, t5215: F, t5318: F, t5321: F, t5326: F, t5354: F, t568: F, t6361: F, t6439: F, t6460: F) -> F {
    let t56607 = t19635 * t225;
    let t56640 = t20048 * t225;
    let t56649 = -F::cast_from(4.0_f64) * t56607 * t1386 + F::cast_from(8.0_f64) * t16030 * t5326 + F::cast_from(4.0_f64) * t5321 * t16471 + F::cast_from(4.0_f64) * t1375 * t3887 * t1842 * t16436 - F::cast_from(12.0_f64) * t5215 * t16475 + F::cast_from(2.0_f64) * t16122 * t1834 * t568 + F::cast_from(4.0_f64) * t5210 * t5318 * t568 + F::cast_from(24.0_f64) * t1375 * t40591 * t6439 * t3888 - F::cast_from(4.0_f64) * t16460 * t5354 + F::cast_from(4.0_f64) * t3758 * t20026 + F::cast_from(8.0_f64) * t16022 * t5326 + F::cast_from(2.0_f64) * t1375 * t3887 * t6460 * t3911 - F::cast_from(2.0_f64) * t56640 * t1386 + t6361 * t3879 * t568 + F::cast_from(8.0_f64) * t16460 * t5326 + F::cast_from(8.0_f64) * t3882 * t19648;
    t56649
}
