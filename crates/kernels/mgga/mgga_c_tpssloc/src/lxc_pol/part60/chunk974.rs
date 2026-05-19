//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 974/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk974<F: Float>(t1888: F, t232: F, t6646: F, t98494: F, t118744: F, t118747: F, t1484: F, t6552: F, t6637: F, t118766: F, t30676: F, t5544: F) -> (F, F, F, F, F) {
    let t126476 = F::cast_from(0.16449340668482264365e-1_f64) * t1888 * t6646 * t98494 * t232;
    let t126477 = F::cast_from(0.15352717957250113407e0_f64) * t118744;
    let t126481 = F::cast_from(0.6579736267392905746e-1_f64) * t6552 * t6637 * t118747 * t1484;
    let t126484 = F::cast_from(0.16449340668482264365e-1_f64) * t118766;
    let t126488 = F::cast_from(0.3289868133696452873e-1_f64) * t6552 * t6637 * t30676 * t5544;
    (t126476, t126477, t126481, t126484, t126488)
}
