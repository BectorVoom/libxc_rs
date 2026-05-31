//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 667/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk667<F: Float>(t52: F, t2244: F, t2250: F, t2440: F, t76: F, t2439: F, t157: F, t182: F, t676: F, t724: F, t164: F, t723: F, t159: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t150 = t52 <= zeta_threshold;
    let t2446 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2440 * t2244 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t2250);
    let t2447 = t2439 + t2446;
    let t2448 = t2447 * t157;
    let t2450 = F::cast_from(0.19751673498613801407e-1_f64) * t2448 * t182;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = F::cast_from(1.0_f64) / t2458;
    let t2460 = t159 * t2459;
    (t2447, t2448, t2450, t2454, t2458, t2459, t2460)
}
