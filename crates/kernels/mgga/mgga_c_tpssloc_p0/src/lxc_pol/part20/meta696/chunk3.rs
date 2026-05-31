//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2657/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2657<F: Float>(t16217: F, t3866: F, t1827: F, t39947: F, t16314: F, t16398: F, t16387: F, t12251: F, t12297: F, t12351: F, t12404: F, t1363: F, t16233: F, t16278: F, t16285: F, t16394: F, t3734: F, t3853: F, t40006: F, t40008: F, t40012: F, t40019: F, t40022: F, t5187: F, t5248: F, t5249: F, t820: F) -> F {
    let t54191 = t3866 * t16217;
    let t54198 = t39947 * t1827;
    let t54199 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54198;
    let t54202 = t16398 * t16314;
    let t54213 = t16398 * t16387;
    let t54215 = -t16278 * t3853 / F::cast_from(1024.0_f64) + t16285 * t12297 / F::cast_from(512.0_f64) + F::cast_from(35.0_f64) / F::cast_from(64.0_f64) * t54191 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t1363 * t12351 * t820 * t5187 * t3734 - t54199 + t16394 * t12404 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t54202 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t40006 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t40008 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t40012 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t40019 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t40022 - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t16233 * t5248 * t5249 * t12251 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t54213;
    t54215
}
