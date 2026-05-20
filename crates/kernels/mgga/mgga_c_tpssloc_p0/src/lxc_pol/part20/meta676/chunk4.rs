//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2554/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2554<F: Float>(t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t44320: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F) -> F {
    let t51769 = F::cast_from(0.4566222222222222222e-1_f64) * t50948;
    let t51785 = F::new(0.30822e0) * t50937 + F::cast_from(0.34246666666666666666e-1_f64) * t50940 + F::new(0.41096e0) * t50946 + t51769 + F::cast_from(0.2283111111111111111e-1_f64) * t50950 + F::cast_from(0.11415555555555555555e-1_f64) * t50952 + F::cast_from(0.6849333333333333333e-1_f64) * t50954 - F::cast_from(0.34246666666666666665e-1_f64) * t50957 - F::cast_from(0.34246666666666666665e-1_f64) * t50961 - F::cast_from(0.20547999999999999999e0_f64) * t50966 + t44320 + F::cast_from(0.2283111111111111111e-1_f64) * t43780 + F::cast_from(0.4566222222222222222e-1_f64) * t43782 + F::cast_from(0.2283111111111111111e-1_f64) * t43784 - F::cast_from(0.34246666666666666665e-1_f64) * t43786 - F::cast_from(0.57077777777777777777e-2_f64) * t43788 - F::cast_from(0.53272592592592592591e-1_f64) * t43816 - F::cast_from(0.20547999999999999999e0_f64) * t50994 + F::new(0.30822e0) * t51000 + F::cast_from(0.57077777777777777775e-1_f64) * t51004;
    t51785
}
