//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2191/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191<F: Float>(t3503: F, t44833: F, t44834: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F, t1227: F, t248: F, t3243: F, t1011: F, t1212: F, t44706: F) -> (F, F, F, F) {
    let t45037 = t44833 * t3503 * t44834;
    let t45044 = t1174 * t2402 * t1197;
    let t45046 = t676 * t3584;
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45080 = t44706 * t1011 * t1212;
    (t45037, t45044, t45049, t45080)
}
