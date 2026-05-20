//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1018/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1018<F: Float>(t4542: F, t974: F, t340: F, t1597: F, t984: F, t343: F, t1593: F, t1600: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2986: F, t4507: F, t4511: F, t4515: F, t4519: F, t4523: F, t4529: F, t4532: F, t973: F) -> (F, F, F, F, F, F) {
    let t4543 = t974 * t4542;
    let t4546 = t974 * t340;
    let t4547 = t1597 * t984;
    let t4548 = t4547 * t343;
    let t4549 = t4546 * t4548;
    let t4552 = -F::cast_from(0.74074074074074074072e-3_f64) * t2958 - t2969 + F::cast_from(0.9259259259259259259e-4_f64) * t2972 - F::cast_from(0.27777777777777777777e-3_f64) * t2975 - F::cast_from(0.74074074074074074072e-3_f64) * t2960 * t1593 + F::cast_from(0.9259259259259259259e-4_f64) * t4507 + F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t4511 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t4515 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4519 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t4523 + F::cast_from(0.22222222222222222222e-2_f64) * t2960 * t1600 - F::cast_from(0.27777777777777777777e-3_f64) * t4529 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t4532 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4543 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4549;
    (t4543, t4546, t4547, t4548, t4549, t4552)
}
