//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 544/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk544<F: Float>(t6591: F, t6593: F, t2229: F, t61: F, t1891: F, t133: F, t119: F, t212: F, t1895: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6594 = t6591 * t6593;
    let t6597 = F::cast_from(1.0_f64) / t61 / t2229;
    let t6598 = t6597 * t1891;
    let t6599 = t6598 * t133;
    let t6600 = t119 * t212;
    let t6601 = t6600 * t1895;
    let t6602 = t6599 * t6601;
    let t6603 = F::cast_from(0.33643963411783659045e-4_f64) * t6602;
    let t6604 = t213 * t225;
    (t6594, t6597, t6598, t6599, t6600, t6601, t6602, t6603, t6604)
}
