//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3178/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178<F: Float>(t13969: F, t19061: F, t3515: F, t11665: F, t11668: F, t11678: F, t11692: F, t1227: F, t14731: F, t14736: F, t14740: F, t15654: F, t1735: F, t19016: F, t19068: F, t3490: F, t3509: F, t3516: F, t3577: F, t3578: F, t4582: F, t4724: F, t4987: F, t5012: F, t52725: F, t52731: F, t52733: F, t55662: F, t55666: F, t5979: F, t62044: F) -> F {
    let t65881 = t3515 * t13969 * t19061;
    let t65883 = F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t11665 * t19016 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3577 * t11668 * t5012 * t4724 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3577 * t11668 * t1735 * t14736 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3577 * t11668 * t1735 * t14740 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t3577 * t11668 * t1735 * t14731 - t11678 * t3578 * t5979 * t3509 / F::cast_from(2304.0_f64) + t11692 * t3578 * t5979 * t3516 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t52725 - t52731 / F::cast_from(3456.0_f64) - t52733 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3490 * t19068 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1227 * t4582 * t4987 * t55666 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t4582 * t4987 * t55662 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1227 * t4582 * t15654 * t62044 - t65881 / F::cast_from(2304.0_f64);
    t65883
}
