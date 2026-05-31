//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1519/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1519<F: Float>(t12291: F, t1341: F, t1343: F, t16285: F, t1827: F, t19855: F, t20492: F, t20497: F, t20556: F, t20570: F, t3790: F, t40449: F, t5235: F, t54020: F, t54793: F, t6417: F, t6422: F, t74290: F, t80076: F, t80085: F, t80189: F, t80193: F, t820: F) -> F {
    let t80474 = -t1341 * t1343 * t820 * t80193 / F::cast_from(3072.0_f64) - t74290 * t1827 / F::cast_from(768.0_f64) - t19855 * t6417 / F::cast_from(512.0_f64) - t5235 * t20556 / F::cast_from(768.0_f64) + t16285 * t20497 / F::cast_from(128.0_f64) - t19855 * t6422 / F::cast_from(512.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t12291 * t1343 * t820 * t80189 - t5235 * t20570 / F::cast_from(768.0_f64) - t54020 * t20492 / F::cast_from(128.0_f64) - t1341 * t1343 * t820 * t80076 / F::cast_from(1024.0_f64) - F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t54793 + t40449 + t3790 * t1343 * t820 * t80085 / F::cast_from(512.0_f64);
    t80474
}
