//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2574/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2574<F: Float>(t43816: F, t44320: F, t51760: F, t51769: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> F {
    let t72037 = -F::cast_from(0.50735802469135802467e-1_f64) * t71166 + F::new(0.30822e0) * t71170 + F::new(0.41096e0) * t71174 + F::cast_from(0.34246666666666666666e-1_f64) * t71179 - F::cast_from(0.34246666666666666665e-1_f64) * t71183 - F::cast_from(0.34246666666666666665e-1_f64) * t71187 + F::new(0.10274e0) * t71191 - F::cast_from(0.20547999999999999999e0_f64) * t71195 - F::cast_from(0.41095999999999999999e0_f64) * t71199 + F::new(0.10274e0) * t71203 + F::new(0.30822e0) * t71206 - t51760 + t51769 + t44320 - F::cast_from(0.17757530864197530864e-1_f64) * t43816 + F::cast_from(0.4566222222222222222e-1_f64) * t63361 + F::cast_from(0.2283111111111111111e-1_f64) * t63382 + F::cast_from(0.6849333333333333333e-1_f64) * t63384 - F::cast_from(0.6849333333333333333e-1_f64) * t63398 - F::new(0.10274e0) * t63400;
    t72037
}
