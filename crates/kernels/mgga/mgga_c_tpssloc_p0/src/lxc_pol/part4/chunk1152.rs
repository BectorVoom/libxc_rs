//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1152/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1152<F: Float>(t18914: F, t18939: F, t475: F, t1214: F, t248: F, t3508: F, t5011: F, t4977: F, t4582: F, t11692: F, t1174: F, t1213: F, t1227: F, t15610: F, t15642: F, t15645: F, t18393: F, t18397: F, t18401: F, t18574: F, t18577: F, t18580: F, t18584: F, t18590: F, t18594: F, t3506: F, t3577: F, t488: F, t4974: F, t4989: F, t5005: F, t5024: F) -> (F, F) {
    let t18940 = t18914 + t18939;
    let t18941 = t18940 * t475;
    let t18943 = t248 * t1214 * t18941;
    let t18946 = t3508 * t5011;
    let t18947 = t4977 * t18946;
    let t18948 = t4582 * t18947;
    let t18951 = -t15610 - t18393 / F::cast_from(3456.0_f64) + t11692 * t18397 / F::cast_from(2304.0_f64) - t3577 * t18401 / F::cast_from(1152.0_f64) + t18574 * t488 / F::cast_from(3072.0_f64) + t1174 * t18577 / F::cast_from(108.0_f64) + t1174 * t18580 / F::cast_from(36.0_f64) - t3577 * t18584 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t5005 * t4989 - t1227 * t18590 / F::cast_from(1152.0_f64) - t1227 * t18594 / F::cast_from(768.0_f64) + t5024 * t4974 / F::cast_from(216.0_f64) + t1213 * t18943 / F::cast_from(3072.0_f64) + t15642 - t15645 + t3506 * t18948 / F::cast_from(768.0_f64);
    (t18940, t18951)
}
