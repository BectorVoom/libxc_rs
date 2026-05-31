//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1079/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1079<F: Float>(t1041: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t17878: F, t17885: F, t17890: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F, t973: F) -> F {
    let t17900 = t14084 + t14508 * t4596 / F::cast_from(768.0_f64) - t14511 * t4600 / F::cast_from(1536.0_f64) + t3130 * t17734 / F::cast_from(768.0_f64) + t973 * t17738 / F::cast_from(288.0_f64) + t17878 * t378 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t3048 * t5861 + F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t17885 + t3117 * t5857 / F::cast_from(4608.0_f64) + t1041 * t17890 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3117 * t5861 + t14085 * t1622 / F::cast_from(2304.0_f64) + t4644 * t4636 / F::cast_from(2304.0_f64) - t14117 / F::cast_from(6912.0_f64);
    t17900
}
