//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1223/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1223<F: Float>(t33334: F, t533: F, t1390: F, t1983: F, t7802: F, t8526: F, t2039: F, t7670: F, t2040: F, t2096: F, t24999: F, t33133: F, t33230: F, t33233: F, t33236: F, t33238: F, t33239: F, t4028: F, t6517: F, t652: F, t7458: F, t7796: F, t7806: F, t8529: F) -> (F, F, F, F) {
    let t33335 = t533 * t33334;
    let t33336 = t33335 * t1390;
    let t33337 = t1983 * t33336;
    let t33345 = F::cast_from(2.0_f64) * t8526 * t7802;
    let t33350 = t7670 * t2039;
    let t33354 = -F::cast_from(2.0_f64) * t2040 * t24999 + t2096 * t33133 - F::cast_from(2.0_f64) * t33350 * t652 - F::cast_from(2.0_f64) * t4028 * t8529 - F::cast_from(2.0_f64) * t6517 * t7796 - F::cast_from(2.0_f64) * t6517 * t7806 - F::cast_from(2.0_f64) * t7458 * t8529 - t33230 - t33233 - t33236 - t33238 + t33239 + t33337 - t33345;
    (t33335, t33336, t33350, t33354)
}
