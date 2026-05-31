//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1321/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1321<F: Float>(t3142: F, t698: F, t973: F, t3147: F, t10981: F, t2960: F, t10263: F, t1041: F, t1044: F, t10860: F, t10957: F, t10972: F, t248: F, t3043: F, t3048: F, t3057: F, t3098: F, t3114: F, t3143: F, t3148: F, t41709: F, t42582: F, t42586: F, t42595: F, t42600: F) -> F {
    let t42610 = t973 * t698 * t3142;
    let t42613 = t973 * t698 * t3147;
    let t42619 = t2960 * t10981;
    let t42621 = -t42582 / F::cast_from(36.0_f64) - t42586 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t3048 * t10972 + t3114 * t10860 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t42595 + F::cast_from(19.0_f64) / F::cast_from(432.0_f64) * t10957 * t3057 - F::cast_from(19.0_f64) / F::cast_from(288.0_f64) * t42600 * t3043 - t1041 * t248 * t1044 * t41709 / F::cast_from(192.0_f64) - F::cast_from(19.0_f64) / F::cast_from(216.0_f64) * t10957 * t3098 - t42610 / F::cast_from(216.0_f64) - t42613 / F::cast_from(162.0_f64) + F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t10263 * t3143 + F::cast_from(22.0_f64) / F::cast_from(81.0_f64) * t10263 * t3148 - t42619 / F::cast_from(27.0_f64);
    t42621
}
