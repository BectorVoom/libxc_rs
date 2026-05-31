//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2039/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2039<F: Float>(t2223: F, t3826: F, t11985: F, t25: F, t514: F, t11998: F, t28: F, t517: F, t32253: F, t59: F, t154: F, t541: F) -> (F, F, F, F, F, F) {
    let t39857 = t2223 * t3826;
    let t39861 = F::cast_from(1.0_f64) / t514 / t11985 / t25;
    let t39877 = F::cast_from(1.0_f64) / t517 / t11998 / t28;
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t541;
    (t39857, t39861, t39877, t39933, t39934, t39936)
}
