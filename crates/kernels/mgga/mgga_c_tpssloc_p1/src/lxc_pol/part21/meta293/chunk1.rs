//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1609/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1609<F: Float>(t10294: F, t10544: F, t2884: F, t922: F, t302: F) -> (F, F, F, F) {
    let t10784 = F::cast_from(0.46308888888888888888e0_f64) * t10294;
    let t10785 = F::cast_from(0.16068111111111111111e1_f64) * t10544;
    let t10810 = F::new(1.0) / t2884 / t922;
    let t10811 = t302 * t10810;
    (t10784, t10785, t10810, t10811)
}
