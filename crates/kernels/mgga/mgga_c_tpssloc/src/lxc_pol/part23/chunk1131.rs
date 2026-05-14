//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1131/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1131<F: Float>(t46657: F, t5593: F, t120: F, t20852: F, t13258: F, t20983: F, t20974: F, t9638: F, t20891: F, t20800: F, t20904: F, t41414: F, t20949: F, t2697: F, t20882: F, t20988: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t67612 = t46657 * t5593;
    let t67620 = t120 * t20852;
    let t67625 = t13258 * t20983;
    let t67637 = t9638 * t20974;
    let t67639 = t9638 * t20891;
    let t67644 = t120 * t20800;
    let t67660 = t41414 * t20904;
    let t67675 = t2697 * t20949;
    let t67690 = t9638 * t20882;
    let t67692 = t13258 * t20988;
    (t67612, t67620, t67625, t67637, t67639, t67644, t67660, t67675, t67690, t67692)
}
