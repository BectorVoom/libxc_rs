//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2423/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423<F: Float>(t48103: F, t49304: F, t49306: F, t49317: F, t49322: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t41684: F, t41863: F, t68460: F, t68464: F, t68468: F, t68472: F, t68479: F, t68483: F, t68486: F, t68489: F, t68492: F, t68494: F) -> (F, F) {
    let t69066 = F::new(0.103295e1) * t68442 + F::cast_from(0.17215833333333333333e0_f64) * t68444 + F::cast_from(0.19128703703703703704e0_f64) * t68446 - F::cast_from(0.68863333333333333333e0_f64) * t68448 + t49304 - t49306 - t49317 - t49322 - F::new(0.41678e0) * t68452 + F::cast_from(0.69463333333333333333e-1_f64) * t68454 + F::cast_from(0.92617777777777777779e0_f64) * t48103;
    let t69079 = F::new(0.62517e0) * t68460 + F::new(0.62517e0) * t68464 - F::new(0.104195e0) * t68468 - F::new(0.104195e0) * t68472 + F::cast_from(0.5356037037037037037e0_f64) * t41684 + F::cast_from(0.30872592592592592592e0_f64) * t41863 - F::cast_from(0.15302962962962962963e1_f64) * t68479 - F::new(0.123954e2) * t68483 + F::new(0.61977e1) * t68486 - F::new(0.103295e1) * t68489 - F::new(0.103295e1) * t68492 + F::cast_from(0.34431666666666666667e0_f64) * t68494;
    (t69066, t69079)
}
