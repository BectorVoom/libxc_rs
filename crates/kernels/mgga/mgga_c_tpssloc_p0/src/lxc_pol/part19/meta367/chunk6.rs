//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1350/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1350<F: Float>(t10432: F, t13969: F, t3039: F, t1021: F, t1025: F, t1041: F, t1044: F, t1046: F, t10863: F, t248: F, t3043: F, t3064: F, t3130: F, t3131: F, t369: F, t378: F, t41671: F, t42422: F, t42729: F, t42731: F, t42735: F, t42743: F, t42746: F, t42752: F, t42756: F, t43083: F, t43094: F, t68: F) -> F {
    let t43097 = t3039 * t13969 * t10432;
    let t43099 = -F::cast_from(5.0_f64) / F::cast_from(216.0_f64) * t10863 * t3064 + t42729 / F::cast_from(576.0_f64) + t42731 / F::cast_from(72.0_f64) + t42735 / F::cast_from(2304.0_f64) + t1041 * t248 * t1044 * t41671 / F::cast_from(4608.0_f64) - t42743 * t3043 / F::cast_from(512.0_f64) + t42746 * t1046 / F::cast_from(1152.0_f64) + t42752 / F::cast_from(3888.0_f64) + t42756 * t1025 / F::cast_from(768.0_f64) + t43083 * t68 * t369 * t378 / F::cast_from(3072.0_f64) + t3130 * t248 * t1021 * t42422 * t3131 / F::cast_from(512.0_f64) + t43094 / F::cast_from(192.0_f64) - t43097 / F::cast_from(384.0_f64);
    t43099
}
