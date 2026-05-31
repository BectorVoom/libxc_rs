//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1485/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485<F: Float>(t12571: F, t1437: F, t19299: F, t20201: F, t20204: F, t20288: F, t2240: F, t39030: F, t39032: F, t39034: F, t39036: F, t39038: F, t39040: F, t39043: F, t39063: F, t3953: F, t45844: F, t5389: F, t5445: F, t55921: F, t605: F, t75284: F, t79579: F, t79585: F, t79637: F, t79707: F, t86: F, t9239: F) -> F {
    let t79711 = (t39030 + t39032 + t39034 + t39036 + t39038 + t39040 + t39043) * t86 - F::cast_from(16.0_f64) * t75284 * t1437 + F::cast_from(120.0_f64) * t55921 * t5389 - F::cast_from(24.0_f64) * t19299 * t5445 - F::cast_from(480.0_f64) * t45844 * t20201 + F::cast_from(240.0_f64) * t12571 * t20204 - F::cast_from(16.0_f64) * t3953 * t20288 + F::cast_from(840.0_f64) * t39063 * t79579 - F::cast_from(720.0_f64) * t9239 * t5389 * t5445 + F::cast_from(60.0_f64) * t2240 * t79585 + F::cast_from(80.0_f64) * t2240 * t1437 * t20288 - F::cast_from(4.0_f64) * t605 * (t79637 + t79707);
    t79711
}
