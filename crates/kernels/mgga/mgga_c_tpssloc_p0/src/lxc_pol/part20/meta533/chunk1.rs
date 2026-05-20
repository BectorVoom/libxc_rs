//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2070/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2070<F: Float>(t40394: F, t40399: F, t535: F, t1317: F, t40005: F, t12189: F, t3745: F, t1314: F, t9580: F, t3741: F, t2566: F, t3732: F) -> (F, F, F, F, F, F) {
    let t40401 = F::cast_from(0.69444444444444444445e-4_f64) * t40394 * t535 * t40399;
    let t40402 = t40005 * t1317;
    let t40404 = t12189 * t3745;
    let t40406 = t9580 * t1314;
    let t40407 = t40406 * t3741;
    let t40409 = t2566 * t3732;
    (t40401, t40402, t40404, t40406, t40407, t40409)
}
