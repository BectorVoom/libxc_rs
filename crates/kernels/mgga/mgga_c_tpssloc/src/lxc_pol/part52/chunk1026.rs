//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1026/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1026<F: Float>(t1060: F, t25499: F, t4688: F, t6800: F, t6799: F, t23665: F, t7611: F, t1936: F, t362: F, t2775: F, t381: F, t3961: F) -> (F, F, F, F, F) {
    let t25500 = t25499 * t1060;
    let t25502 = t4688 * t6800;
    let t25503 = t6799 * t25502;
    let t25508 = t23665 * t7611;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25512 = t25511 * t3961;
    (t25500, t25503, t25508, t25510, t25512)
}
