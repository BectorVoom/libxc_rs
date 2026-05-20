//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2905/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2905<F: Float>(t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F) -> F {
    let t60585 = F::cast_from(0.83356000000000000002e0_f64) * t48087 + F::cast_from(0.18363555555555555555e1_f64) * t47705 - F::cast_from(0.6121185185185185185e0_f64) * t47707 + F::cast_from(0.45908888888888888888e0_f64) * t47709 + F::cast_from(0.22954444444444444444e0_f64) * t47711 + F::cast_from(0.38257407407407407407e0_f64) * t47713 - F::cast_from(0.13772666666666666666e1_f64) * t47715 - F::cast_from(0.68863333333333333332e0_f64) * t47717 - F::cast_from(0.13772666666666666666e1_f64) * t47724 + F::cast_from(0.23154444444444444444e0_f64) * t41831 - F::cast_from(0.4630888888888888889e0_f64) * t48096 + F::cast_from(0.13892666666666666667e0_f64) * t48098 - F::cast_from(0.91817777777777777776e0_f64) * t47730 + F::cast_from(0.34431666666666666666e0_f64) * t47732;
    t60585
}
