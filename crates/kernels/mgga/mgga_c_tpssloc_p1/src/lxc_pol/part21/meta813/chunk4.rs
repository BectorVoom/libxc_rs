//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2861/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2861<F: Float>(t42245: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F) -> F {
    let t59815 = F::cast_from(0.1522074074074074074e-1_f64) * t59688 + F::cast_from(0.68493333333333333332e-1_f64) * t59692 - F::cast_from(0.76103703703703703702e-2_f64) * t59694 + t42245 + F::cast_from(0.34246666666666666666e-1_f64) * t59698 - F::cast_from(0.4566222222222222222e-1_f64) * t59700 + F::cast_from(0.1522074074074074074e-1_f64) * t59702 + F::cast_from(0.12683950617283950617e-1_f64) * t59704 - F::cast_from(0.19025925925925925925e-1_f64) * t59708 - F::cast_from(0.50735802469135802467e-1_f64) * t59713 + F::cast_from(0.68493333333333333331e-1_f64) * t59717 - F::cast_from(0.2283111111111111111e-1_f64) * t59721;
    t59815
}
