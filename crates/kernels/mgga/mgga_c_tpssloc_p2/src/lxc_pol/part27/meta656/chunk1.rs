//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2291/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2291<F: Float>(t1985: F, t22934: F, t26193: F, t80722: F, t80725: F, t80728: F, t80738: F, t80744: F, t90598: F, t90602: F, t90605: F, t90609: F, t90612: F) -> F {
    let t90615 = t1985 * t26193 * t22934;
    let t90617 = F::cast_from(0.12793931631041761173e0_f64) * t80722;
    let t90621 = -F::cast_from(0.16449340668482264365e-1_f64) * t90598 - F::cast_from(0.6579736267392905746e-1_f64) * t90602 - t90605 - F::cast_from(0.49348022005446793096e-1_f64) * t90609 + F::cast_from(0.3289868133696452873e-1_f64) * t90612 + F::cast_from(0.16449340668482264365e-1_f64) * t90615 + t90617 + F::cast_from(0.41123351671205660912e-2_f64) * t80725 - F::cast_from(0.11514538467937585055e0_f64) * t80728 - F::cast_from(0.41123351671205660912e-2_f64) * t80738 - t80744;
    t90621
}
