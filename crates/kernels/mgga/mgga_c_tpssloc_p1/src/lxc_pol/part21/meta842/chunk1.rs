//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3036/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3036<F: Float>(t43748: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t63327: F, t63330: F, t63332: F, t63334: F, t63336: F) -> F {
    let t63346 = F::cast_from(0.41096e0_f64) * t63327 - F::cast_from(0.27397333333333333332e0_f64) * t63330 - F::cast_from(0.50735802469135802467e-2_f64) * t63332 + F::cast_from(0.76103703703703703702e-2_f64) * t63334 - F::cast_from(0.11415555555555555555e-1_f64) * t63336 - F::cast_from(0.50735802469135802469e-2_f64) * t43748 - F::cast_from(0.4566222222222222222e-1_f64) * t50903 - F::cast_from(0.2283111111111111111e-1_f64) * t50905 - F::cast_from(0.6849333333333333333e-1_f64) * t50907 - F::cast_from(0.20294320987654320986e-1_f64) * t50919 - F::cast_from(0.12683950617283950617e-1_f64) * t50921 + F::cast_from(0.6088296296296296296e-1_f64) * t50948 + F::cast_from(0.1522074074074074074e-1_f64) * t50950;
    t63346
}
