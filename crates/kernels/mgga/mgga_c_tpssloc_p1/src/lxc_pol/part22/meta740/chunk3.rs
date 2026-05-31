//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2440/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440<F: Float>(t42245: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> F {
    let t69445 = F::cast_from(0.10274e0_f64) * t68586 + F::cast_from(0.34246666666666666666e-1_f64) * t68589 - F::cast_from(0.11415555555555555555e-1_f64) * t68592 + F::cast_from(0.2283111111111111111e0_f64) * t68596 - F::cast_from(0.57077777777777777775e-1_f64) * t68599 + F::cast_from(0.20547999999999999999e0_f64) * t68602 - F::cast_from(0.57077777777777777775e-1_f64) * t68605 - F::cast_from(0.30822e0_f64) * t68608 - F::cast_from(0.34246666666666666666e-1_f64) * t59663 + F::cast_from(0.11415555555555555555e-1_f64) * t59665 + F::cast_from(0.17123333333333333333e-1_f64) * t59680 + F::cast_from(0.4566222222222222222e-1_f64) * t59688 - F::cast_from(0.22831111111111111111e-1_f64) * t59694 + t42245 - F::cast_from(0.6849333333333333333e-1_f64) * t59700 + F::cast_from(0.2283111111111111111e-1_f64) * t59702 + F::cast_from(0.19025925925925925925e-1_f64) * t59704 + F::cast_from(0.5327259259259259259e-1_f64) * t47787 + F::cast_from(0.10274e0_f64) * t59759 - F::cast_from(0.6849333333333333333e-1_f64) * t59761;
    t69445
}
