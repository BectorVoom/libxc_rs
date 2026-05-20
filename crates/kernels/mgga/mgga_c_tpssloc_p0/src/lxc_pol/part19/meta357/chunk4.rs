//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1298/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298<F: Float>(t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F) -> F {
    let t42212 = F::cast_from(0.5356037037037037037e1_f64) * t41654;
    let t42213 = F::cast_from(0.16979925925925925926e1_f64) * t41961;
    let t42218 = -F::cast_from(0.6618234375e1_f64) * t41937 - F::new(0.52945875e1) * t41940 + F::cast_from(0.2366859375e0_f64) * t41943 + F::new(0.94674375e0) * t41945 - F::new(0.705945e1) * t41948 + F::new(0.1262325e1) * t41951 + F::cast_from(0.158837625e2_f64) * t41954 - F::new(0.94674375e0) * t41957 + t42212 + t42213 - F::cast_from(0.13892666666666666667e0_f64) * t41964 - F::cast_from(0.27785333333333333334e0_f64) * t41967 - F::new(0.375102e1) * t41970 + F::new(0.83356e0) * t41973;
    t42218
}
