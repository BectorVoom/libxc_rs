//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1616/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1616<F: Float>(t11778: F, t61: F, t11148: F, t248: F, t121: F, t3584: F, t3243: F, t1227: F, t1229: F, t676: F) -> (F, F, F, F, F, F) {
    let t11779 = t61 * t11778;
    let t11781 = t248 * t11779 * t11148;
    let t11784 = t121 * t3584;
    let t11786 = t248 * t11784 * t3243;
    let t11787 = t1227 * t11786;
    let t11789 = t676 * t1229;
    (t11779, t11781, t11784, t11786, t11787, t11789)
}
