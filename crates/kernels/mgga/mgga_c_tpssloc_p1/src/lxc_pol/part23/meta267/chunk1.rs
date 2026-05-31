//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 940/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk940<F: Float>(t28: F, t12072: F, t20385: F, t20390: F, t5142: F, t517: F, t5966: F, t157: F, t20384: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t20394 = piecewise3::<F>(t29, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12072 * t20385 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5142 * t5966 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t20390);
    let t20396 = (t20384 + t20394) * t157;
    t20396
}
