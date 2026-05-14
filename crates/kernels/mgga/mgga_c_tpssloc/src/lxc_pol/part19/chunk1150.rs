//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1150/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1150<F: Float>(t10046: F, t814: F, t10016: F, t10058: F, t10073: F, t10081: F, t10094: F, t13453: F, t255: F, t2613: F, t2617: F, t2728: F, t2732: F, t2740: F, t41231: F, t41333: F, t41368: F, t41429: F, t808: F, t812: F, t860: F, t863: F, t9661: F) -> (F, F) {
    let t41520 = t814 * t10046;
    let t41549 = 6.0 * t2728 * t41368 * t812 - 4.0 * t2732 * t812 * t9661 - t41333 * t812 * t860 - 3.0 * t41429 * t812 * t860 + 4.0 * t10016 * t863 + 4.0 * t10058 * t808 - 12.0 * t10073 * t2617 - 24.0 * t10081 * t2617 + 24.0 * t10094 * t13453 + t255 * t41231 + 6.0 * t2613 * t2740;
    (t41520, t41549)
}
