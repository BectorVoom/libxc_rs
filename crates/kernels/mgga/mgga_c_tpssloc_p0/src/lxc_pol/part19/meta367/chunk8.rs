//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1352/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352<F: Float>(t1025: F, t10403: F, t10426: F, t10428: F, t10480: F, t10501: F, t10517: F, t10915: F, t10949: F, t10965: F, t13980: F, t13985: F, t14213: F, t3071: F, t3098: F, t3117: F, t3123: F, t3130: F, t39110: F, t42639: F, t43103: F, t43110: F, t43114: F, t43118: F, t43120: F, t4582: F, t4594: F, t973: F, t974: F, t998: F) -> F {
    let t43141 = -t3117 * t10915 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t43103 + t973 * t974 * t998 * t39110 / F::cast_from(288.0_f64) + t43110 / F::cast_from(108.0_f64) + F::cast_from(19.0_f64) / F::cast_from(288.0_f64) * t10517 * t3123 - t43114 / F::cast_from(1728.0_f64) + t43118 / F::cast_from(1152.0_f64) - t43120 * t1025 / F::cast_from(48.0_f64) + t10949 * t10428 / F::cast_from(128.0_f64) + t3130 * t4582 * t42639 * t4594 / F::cast_from(384.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t10480 * t4582 * t10426 * t13985 - t10965 * t3098 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(576.0_f64) * t3117 * t10501 + t10403 * t3071 * t13980 * t14213 / F::cast_from(192.0_f64);
    t43141
}
