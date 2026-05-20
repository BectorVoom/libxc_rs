//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1992/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1992<F: Float>(t15621: F, t4582: F, t11721: F, t3507: F, t4977: F, t3509: F, t1216: F, t15553: F, t13969: F, t4979: F, t3506: F, t4973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15622 = t4582 * t15621;
    let t15625 = t11721 * t3507;
    let t15626 = t4977 * t15625;
    let t15627 = t4582 * t15626;
    let t15630 = t4977 * t3509;
    let t15631 = t4582 * t15630;
    let t15636 = t15553 * t1216;
    let t15637 = t4582 * t15636;
    let t15640 = t13969 * t4979;
    let t15642 = t3506 * t15640 / F::new(1152.0);
    let t15643 = t13969 * t4973;
    (t15622, t15625, t15626, t15627, t15630, t15631, t15636, t15637, t15640, t15642, t15643)
}
