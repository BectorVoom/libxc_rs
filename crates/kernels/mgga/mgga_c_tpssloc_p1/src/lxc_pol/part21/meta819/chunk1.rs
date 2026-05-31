//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2884/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884<F: Float>(t13637: F, t60237: F, t41959: F, t41962: F, t59680: F, t59684: F, t59688: F, t59692: F, t59694: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F, t60238: F) -> (F, F) {
    let t60240 = t13637 * t60237;
    let t60242 = F::cast_from(0.20128333333333333334e0_f64) * t59680 - F::cast_from(0.301925e0_f64) * t59684 + F::cast_from(0.26837777777777777777e0_f64) * t59688 + F::cast_from(0.12077e1_f64) * t59692 - F::cast_from(0.13418888888888888889e0_f64) * t59694 - F::cast_from(0.5519e-1_f64) * t60223 - F::cast_from(0.27595e-1_f64) * t60226 - F::cast_from(0.36793333333333333333e-1_f64) * t60229 - F::cast_from(0.99342e0_f64) * t60232 - F::cast_from(0.49671e0_f64) * t60235 + t41959 + t41962 + F::cast_from(0.776775e1_f64) * t60238 - F::cast_from(0.16504875e0_f64) * t60240;
    (t60240, t60242)
}
