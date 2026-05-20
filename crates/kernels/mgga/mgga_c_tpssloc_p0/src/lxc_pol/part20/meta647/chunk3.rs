//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2378/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2378<F: Float>(t2885: F, t4408: F, t47705: F, t47707: F, t47730: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47732: F, t47736: F, t47738: F) -> (F, F) {
    let t48789 = t4408 * t2885;
    let t48799 = F::cast_from(0.4566222222222222222e-1_f64) * t47705;
    let t48800 = F::cast_from(0.1522074074074074074e-1_f64) * t47707;
    let t48809 = F::cast_from(0.2283111111111111111e-1_f64) * t47730;
    let t48813 = -F::cast_from(0.50735802469135802467e-1_f64) * t47681 + F::cast_from(0.20547999999999999999e0_f64) * t47686 - F::cast_from(0.34246666666666666665e-1_f64) * t47691 - F::cast_from(0.34246666666666666665e-1_f64) * t47695 - F::cast_from(0.11415555555555555555e-1_f64) * t47699 - F::new(0.30822e0) * t47703 + t48799 - t48800 + F::cast_from(0.2283111111111111111e-1_f64) * t47709 + F::cast_from(0.11415555555555555555e-1_f64) * t47711 + F::cast_from(0.19025925925925925925e-1_f64) * t47713 - F::cast_from(0.68493333333333333331e-1_f64) * t47715 - F::cast_from(0.34246666666666666665e-1_f64) * t47717 - F::cast_from(0.57077777777777777775e-1_f64) * t47722 - F::cast_from(0.6849333333333333333e-1_f64) * t47724 - F::cast_from(0.41095999999999999999e0_f64) * t47728 - t48809 + F::cast_from(0.17123333333333333333e-1_f64) * t47732 - F::cast_from(0.17123333333333333333e-1_f64) * t47736 + F::new(0.10274e0) * t47738;
    (t48789, t48813)
}
