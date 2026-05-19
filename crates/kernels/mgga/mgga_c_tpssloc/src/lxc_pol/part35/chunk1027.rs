//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1027/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1027<F: Float>(t10214: F, t21468: F, t20234: F, t2980: F, t977: F, t21126: F, t4518: F, t13909: F, t17784: F, t17809: F, t21430: F, t21433: F, t21447: F, t21453: F, t21459: F, t21463: F, t2986: F, t973: F) -> F {
    let t21469 = t10214 * t21468;
    let t21472 = t2980 * t20234;
    let t21473 = t977 * t21472;
    let t21476 = t4518 * t21126;
    let t21479 = F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t21430 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t21433 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t21447 - F::cast_from(0.55555555555555555554e-3_f64) * t17809 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t21453 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t21459 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t21463 + F::cast_from(0.37037037037037037036e-3_f64) * t17784 + F::cast_from(0.55555555555555555554e-3_f64) * t13909 + F::cast_from(0.86419753086419753084e-3_f64) * t973 * t21469 + F::cast_from(0.16666666666666666666e-2_f64) * t973 * t21473 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t21476;
    t21479
}
