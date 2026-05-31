//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1285/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285<F: Float>(t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41959: F, t41962: F, t41964: F, t41967: F, t41970: F, t41973: F) -> F {
    let t41975 = -F::cast_from(0.485484375e1_f64) * t41937 - F::cast_from(0.3883875e1_f64) * t41940 + F::cast_from(0.6189328125e-1_f64) * t41943 + F::cast_from(0.247573125e0_f64) * t41945 - F::cast_from(0.51785e1_f64) * t41948 + F::cast_from(0.3300975e0_f64) * t41951 + F::cast_from(0.11651625e2_f64) * t41954 - F::cast_from(0.247573125e0_f64) * t41957 + t41959 + t41962 - F::cast_from(0.11038e0_f64) * t41964 - F::cast_from(0.22076e0_f64) * t41967 - F::cast_from(0.298026e1_f64) * t41970 + F::cast_from(0.66228e0_f64) * t41973;
    t41975
}
