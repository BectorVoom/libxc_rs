//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1086/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1086<F: Float>(t1229: F, t204: F, t11604: F, t496: F, t68: F, t107: F, t9576: F, t106: F, t9364: F, t35761: F, t35577: F, t1454: F, t2585: F, t1406: F, t9238: F, t4199: F, t9919: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45293 = t204 * t1229;
    let t45349 = 1.0 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45421 = 2618.0 / 81.0 * t9576 * t107;
    let t45435 = 1.0 / t9364 / t106;
    let t45460 = 1.0 / t35761;
    let t45496 = 1.0 / t35577;
    let t45656 = t2585 * t1454;
    let t45844 = t1406 * t9238;
    let t46125 = t4199 * t9919;
    (t45293, t45350, t45421, t45435, t45460, t45496, t45656, t45844, t46125)
}
