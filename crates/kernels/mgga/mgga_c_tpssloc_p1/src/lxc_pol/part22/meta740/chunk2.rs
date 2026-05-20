//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2439/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439<F: Float>(t21194: F, t2888: F, t41684: F, t48799: F, t48800: F, t48809: F, t59657: F, t68442: F, t68444: F, t68446: F, t68448: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F, t68498: F, t68571: F, t68577: F, t68580: F, t68583: F) -> (F, F) {
    let t69380 = t21194 * t2888;
    let t69425 = F::cast_from(0.34246666666666666667e-1_f64) * t68442 + F::cast_from(0.57077777777777777777e-2_f64) * t68444 + F::cast_from(0.63419753086419753083e-2_f64) * t68446 - F::cast_from(0.2283111111111111111e-1_f64) * t68448 + t48799 - t48800 - t48809 + F::cast_from(0.17757530864197530864e-1_f64) * t41684 - F::cast_from(0.50735802469135802467e-1_f64) * t68479 - F::cast_from(0.41095999999999999999e0_f64) * t68483 + F::cast_from(0.20547999999999999999e0_f64) * t68486 - F::cast_from(0.34246666666666666665e-1_f64) * t68489 - F::cast_from(0.34246666666666666665e-1_f64) * t68492 + F::cast_from(0.11415555555555555555e-1_f64) * t68494 - F::cast_from(0.34246666666666666667e-1_f64) * t68498 - F::cast_from(0.1522074074074074074e-1_f64) * t59657 - F::cast_from(0.17123333333333333333e-1_f64) * t68571 + F::new(0.41096e0) * t68577 - F::new(0.30822e0) * t68580 + F::new(0.10274e0) * t68583;
    (t69380, t69425)
}
