//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 663/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk663<F: Float>(t2990: F, t4531: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F, t974: F, t1597: F, t984: F, t1593: F, t1600: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2986: F, t4507: F, t4511: F, t4515: F, t4519: F, t4523: F, t4529: F, t973: F) -> (F, F, F, F, F) {
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / 9.0 - t4384 / 9.0 + t4387 / 18.0 - t4390 / 3.0 + t4393 / 6.0;
    let t4541 = t340 * t4540;
    let t4542 = t4541 * t343;
    let t4543 = t974 * t4542;
    let t4546 = t974 * t340;
    let t4547 = t1597 * t984;
    let t4548 = t4547 * t343;
    let t4549 = t4546 * t4548;
    let t4552 = -0.74074074074074074072e-3 * t2958 - t2969 + 0.9259259259259259259e-4 * t2972 - 0.27777777777777777777e-3 * t2975 - 0.74074074074074074072e-3 * t2960 * t1593 + 0.9259259259259259259e-4 * t4507 + 0.37037037037037037036e-3 * t2986 * t4511 - 0.27777777777777777777e-3 * t2986 * t4515 - 0.55555555555555555554e-3 * t2986 * t4519 + 0.27777777777777777777e-3 * t973 * t4523 + 0.22222222222222222222e-2 * t2960 * t1600 - 0.27777777777777777777e-3 * t4529 - 0.27777777777777777777e-3 * t2986 * t4532 - 0.83333333333333333332e-3 * t973 * t4543 - 0.83333333333333333332e-3 * t973 * t4549;
    (t4540, t4542, t4546, t4548, t4552)
}
