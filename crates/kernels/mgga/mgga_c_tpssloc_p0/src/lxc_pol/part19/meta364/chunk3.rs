//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1329/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329<F: Float>(t42661: F, t42679: F, t42693: F, t42713: F, t10510: F, t3114: F, t1020: F, t1021: F, t1023: F, t1025: F, t1041: F, t10426: F, t10433: F, t1046: F, t10463: F, t10863: F, t10876: F, t10952: F, t14164: F, t248: F, t3039: F, t3048: F, t3057: F, t3132: F, t360: F, t39097: F, t42468: F, t42622: F, t42624: F, t42639: F, t42648: F, t42651: F, t42653: F, t42658: F, t4582: F, t973: F, t974: F) -> (F, F) {
    let t42715 = t42661 + t42679 + t42693 + t42713;
    let t42721 = t3114 * t10510;
    let t42723 = -F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t42622 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t973 * t974 * t42624 * t39097 - t10863 * t3057 / F::cast_from(72.0_f64) - t3048 * t10463 / F::cast_from(216.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t10876 * t4582 * t10426 * t3132 - t10952 * t10433 / F::cast_from(256.0_f64) - t3039 * t4582 * t42639 * t1023 / F::cast_from(768.0_f64) + t1041 * t4582 * t14164 * t42468 / F::cast_from(128.0_f64) + F::cast_from(19.0_f64) / F::cast_from(216.0_f64) * t42648 * t1046 - t42651 / F::cast_from(54.0_f64) + F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t42653 * t1025 - F::cast_from(209.0_f64) / F::cast_from(648.0_f64) * t42658 * t1025 + t1020 * t248 * t1021 * t42715 * t360 / F::cast_from(3072.0_f64) - t42721 / F::cast_from(1152.0_f64);
    (t42715, t42723)
}
