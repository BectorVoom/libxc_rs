//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1291/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291<F: Float>(t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F) -> F {
    let t42086 = F::cast_from(0.31003950617283950618e1_f64) * t41654;
    let t42087 = F::cast_from(0.13388493827160493828e1_f64) * t41961;
    let t42092 = -F::cast_from(0.3560484375e1_f64) * t41937 - F::new(0.28483875e1) * t41940 + F::cast_from(0.1151859375e0_f64) * t41943 + F::new(0.46074375e0) * t41945 - F::new(0.379785e1) * t41948 + F::new(0.614325e0) * t41951 + F::new(0.85451625e1) * t41954 - F::new(0.46074375e0) * t41957 + t42086 + t42087 - F::cast_from(0.10954222222222222222e0_f64) * t41964 - F::cast_from(0.21908444444444444444e0_f64) * t41967 - F::new(0.295764e1) * t41970 + F::cast_from(0.65725333333333333332e0_f64) * t41973;
    t42092
}
