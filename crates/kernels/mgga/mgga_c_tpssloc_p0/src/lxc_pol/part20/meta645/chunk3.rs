//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2371/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2371<F: Float>(t47730: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47732: F, t47736: F, t47738: F, t48688: F, t48689: F) -> F {
    let t48698 = F::cast_from(0.23744444444444444444e-1_f64) * t47730;
    let t48702 = -F::cast_from(0.52765432098765432099e-1_f64) * t47681 + F::new(0.2137e0) * t47686 - F::cast_from(0.35616666666666666666e-1_f64) * t47691 - F::cast_from(0.35616666666666666666e-1_f64) * t47695 - F::cast_from(0.11872222222222222222e-1_f64) * t47699 - F::new(0.32055e0) * t47703 + t48688 - t48689 + F::cast_from(0.23744444444444444444e-1_f64) * t47709 + F::cast_from(0.11872222222222222222e-1_f64) * t47711 + F::cast_from(0.19787037037037037036e-1_f64) * t47713 - F::cast_from(0.71233333333333333332e-1_f64) * t47715 - F::cast_from(0.35616666666666666666e-1_f64) * t47717 - F::cast_from(0.5936111111111111111e-1_f64) * t47722 - F::cast_from(0.71233333333333333331e-1_f64) * t47724 - F::cast_from(0.42739999999999999999e0_f64) * t47728 - t48698 + F::cast_from(0.17808333333333333333e-1_f64) * t47732 - F::cast_from(0.17808333333333333333e-1_f64) * t47736 + F::new(0.10685e0) * t47738;
    t48702
}
