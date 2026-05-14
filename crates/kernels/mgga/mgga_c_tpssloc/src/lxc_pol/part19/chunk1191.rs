//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1191/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1191<F: Float>(t10704: F, t41995: F, t42028: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41845: F) -> (F, F) {
    let t42031 = 0.62071215503128080361e4 * t42028 * t41995 * t10704;
    let t42046 = 0.10954222222222222222e1 * t41831 + 0.13145066666666666666e1 * t41833 - 0.98587999999999999998e0 * t41836 - 0.82156666666666666668e-1 * t41839 + 0.197176e1 * t41842 + 0.49293999999999999999e0 * t41845 + 0.17938e1 * t41642 - 0.79724444444444444446e0 * t41656 - 0.5314962962962962963e0 * t41658 + 0.44291358024691358024e0 * t41660 + 0.39862222222222222223e0 * t41662 - 0.88582716049382716048e0 * t41669 - 0.29896666666666666667e0 * t41673 + 0.15944888888888888889e1 * t41675;
    (t42031, t42046)
}
