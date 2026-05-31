//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2972/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972<F: Float>(t10952: F, t17655: F, t17659: F, t3117: F, t1041: F, t17187: F, t248: F, t3051: F, t10390: F, t10480: F, t10904: F, t13762: F, t13995: F, t14488: F, t17670: F, t17714: F, t17998: F, t3040: F, t3071: F, t3130: F, t3131: F, t42552: F, t42573: F, t43291: F, t43292: F, t4582: F, t4593: F, t4596: F, t48607: F, t49651: F, t49682: F, t49684: F, t50510: F, t5880: F, t61078: F) -> F {
    let t61975 = t10952 * t17655;
    let t61977 = t3117 * t17659;
    let t61981 = t1041 * t248 * t3051 * t17187;
    let t62007 = F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t10390 * t17998 + t13995 * t13762 / F::cast_from(1152.0_f64) + t42573 * t5880 / F::cast_from(288.0_f64) - t61975 / F::cast_from(2304.0_f64) + t61977 / F::cast_from(3456.0_f64) + t61981 / F::cast_from(3456.0_f64) + t10480 * t4582 * t17670 * t50510 / F::cast_from(512.0_f64) + t43291 * t4582 * t17670 * t43292 * t3040 / F::cast_from(128.0_f64) + t3130 * t4582 * t4593 * t3131 * t14488 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t42552 - t10904 * t17714 / F::cast_from(144.0_f64) + t49682 / F::cast_from(1728.0_f64) + t48607 * t3071 * t61078 / F::cast_from(192.0_f64) + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t49684 + t49651 * t4596 / F::cast_from(384.0_f64);
    t62007
}
