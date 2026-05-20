//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2859/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2859<F: Float>(t324: F, t59650: F, t59686: F, t59723: F, t59771: F, t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F) -> (F, F) {
    let t59774 = (t59650 + t59686 + t59723 + t59771) * t324;
    let t59788 = F::cast_from(0.6088296296296296296e-1_f64) * t47705 - F::cast_from(0.20294320987654320986e-1_f64) * t47707 + F::cast_from(0.1522074074074074074e-1_f64) * t47709 + F::cast_from(0.761037037037037037e-2_f64) * t47711 + F::cast_from(0.12683950617283950617e-1_f64) * t47713 - F::cast_from(0.4566222222222222222e-1_f64) * t47715 - F::cast_from(0.2283111111111111111e-1_f64) * t47717 - F::cast_from(0.4566222222222222222e-1_f64) * t47724 - F::cast_from(0.3044148148148148148e-1_f64) * t47730 + F::cast_from(0.11415555555555555555e-1_f64) * t47732 + F::cast_from(0.6849333333333333333e-1_f64) * t47738 - F::cast_from(0.76103703703703703703e-2_f64) * t41656;
    (t59774, t59788)
}
