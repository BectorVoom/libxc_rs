//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1522/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522<F: Float>(t6439: F, t12021: F, t1375: F, t1807: F, t1843: F, t20044: F, t20060: F, t20601: F, t20609: F, t20662: F, t40591: F, t5215: F, t5321: F, t539: F, t568: F, t6440: F, t6460: F, t6461: F, t74860: F, t74908: F, t80477: F) -> F {
    let t80511 = t6439 * t6439;
    let t80521 = -F::cast_from(36.0_f64) * t12021 * t1375 * t6439 * t6460 + F::cast_from(24.0_f64) * t1375 * t40591 * t80511 + F::cast_from(4.0_f64) * t1807 * t20601 * t568 + t539 * t568 * t80477 - F::cast_from(12.0_f64) * t1843 * t74860 - F::cast_from(12.0_f64) * t1843 * t74908 + F::cast_from(12.0_f64) * t20044 * t6440 - F::cast_from(6.0_f64) * t20044 * t6461 + F::cast_from(12.0_f64) * t20060 * t6440 - F::cast_from(24.0_f64) * t20609 * t5215 - F::cast_from(24.0_f64) * t20609 * t5321 - F::cast_from(4.0_f64) * t20662 * t5215 - F::cast_from(4.0_f64) * t20662 * t5321;
    t80521
}
