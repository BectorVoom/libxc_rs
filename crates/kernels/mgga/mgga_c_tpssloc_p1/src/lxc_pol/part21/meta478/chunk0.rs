//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2070/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2070<F: Float>(t12832: F, t16505: F, t3: F, t112: F, t5363: F, t111: F, t1851: F, t2319: F, t576: F, t4072: F, t671: F, t1458: F, t2363: F) -> (F, F, F, F, F, F, F) {
    let t16506 = t12832 + t16505;
    let t16507 = t3 * t16506;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16535 = t576 * t2319;
    let t16538 = t4072 * t671;
    let t16541 = t1458 * t2363;
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541)
}
