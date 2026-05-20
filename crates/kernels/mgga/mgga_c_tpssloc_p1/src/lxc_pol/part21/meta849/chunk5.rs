//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3077/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077<F: Float>(t423: F, t63784: F, t63798: F, t63811: F, t63825: F, t18496: F, t699: F, t18517: F, t18514: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F) -> (F, F, F, F, F) {
    let t63829 = F::new(0.621814e-1) * (t63784 + t63798 + t63811 + t63825) * t423;
    let t63841 = t699 * t18496;
    let t63843 = t699 * t18517;
    let t63845 = t699 * t18514;
    let t63847 = -F::cast_from(0.39862222222222222222e0_f64) * t63291 + F::cast_from(0.11958666666666666667e1_f64) * t63296 + F::cast_from(0.59793333333333333334e0_f64) * t63300 + F::new(0.17938e1) * t63304 + F::cast_from(0.13287407407407407408e0_f64) * t63306 - F::cast_from(0.22145679012345679012e0_f64) * t63308 - F::cast_from(0.39862222222222222222e0_f64) * t63313 - F::cast_from(0.19931111111111111111e0_f64) * t63317 + F::cast_from(0.5314962962962962963e0_f64) * t50826 - F::cast_from(0.19931111111111111111e0_f64) * t50828 - F::cast_from(0.62007901234567901235e0_f64) * t50834 - F::cast_from(0.48685432098765432099e-1_f64) * t63841 - F::cast_from(0.21908444444444444444e0_f64) * t63843 + F::cast_from(0.36514074074074074074e-1_f64) * t63845;
    (t63829, t63841, t63843, t63845, t63847)
}
