//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1762/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1762<F: Float>(t1851: F, t2319: F, t2363: F, t576: F, t4025: F, t671: F, t1441: F, t1799: F, t3914: F, t1388: F, t5187: F, t1307: F, t5356: F) -> (F, F, F, F, F, F, F) {
    let t55405 = t1851 * t2319;
    let t55571 = t576 * t2363;
    let t55934 = t4025 * t671;
    let t55962 = t1441 * t2363;
    let t56120 = t1799 * t3914;
    let t56194 = t5187 * t1388;
    let t56198 = t1307 * t5356;
    (t55405, t55571, t55934, t55962, t56120, t56194, t56198)
}
