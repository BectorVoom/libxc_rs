//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2198/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2198<F: Float>(t11148: F, t1227: F, t248: F, t45268: F, t11728: F, t11729: F, t3570: F, t1229: F, t204: F, t1090: F, t3609: F, t44927: F) -> (F, F, F, F, F) {
    let t45271 = t1227 * t248 * t45268 * t11148;
    let t45283 = t11728 * t248 * t3570 * t11729;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45320 = t44927 * t3609;
    (t45271, t45283, t45293, t45296, t45320)
}
