//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1646/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1646<F: Float>(t11135: F, t11203: F, t1127: F, t3355: F, t427: F) -> (F, F, F, F) {
    let t11314 = F::cast_from(0.16068111111111111111e1_f64) * t11135;
    let t11317 = F::cast_from(0.46308888888888888888e0_f64) * t11203;
    let t11349 = F::new(1.0) / t3355 / t1127;
    let t11350 = t427 * t11349;
    (t11314, t11317, t11349, t11350)
}
