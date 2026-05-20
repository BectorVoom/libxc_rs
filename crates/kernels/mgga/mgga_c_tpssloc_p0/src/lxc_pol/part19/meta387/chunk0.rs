//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1455/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1455<F: Float>(t42339: F, t466: F, t11715: F, t42341: F, t3507: F, t491: F, t11721: F, t23508: F, t1009: F, t11598: F, t1243: F, t3590: F) -> (F, F, F, F, F, F, F, F) {
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    let t44699 = t3507 * t3507;
    let t44700 = t491 * t44699;
    let t44701 = t23508 * t11721;
    let t44706 = t11598 * t1009;
    let t44707 = t44706 * t1243;
    let t44710 = t3590 * t3507;
    (t44696, t44698, t44699, t44700, t44701, t44706, t44707, t44710)
}
