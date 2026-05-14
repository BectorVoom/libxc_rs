//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1175/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1175<F: Float>(t106: F, t9364: F, t111: F, t3931: F, t12723: F, t112: F, t16506: F, t5363: F, t1851: F, t2319: F, t2363: F, t576: F, t4025: F, t671: F, t1441: F, t1395: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t45435 = 1.0 / t9364 / t106;
    let t45560 = t3931 * t111;
    let t45632 = t12723 * t111;
    let t55341 = t16506 * t112;
    let t55353 = t5363 * t111;
    let t55405 = t1851 * t2319;
    let t55571 = t576 * t2363;
    let t55934 = t4025 * t671;
    let t55962 = t1441 * t2363;
    let t66940 = t1395 * t671;
    (t45435, t45560, t45632, t55341, t55353, t55405, t55571, t55934, t55962, t66940)
}
