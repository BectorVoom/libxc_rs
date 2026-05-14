//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1185/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1185<F: Float>(t10213: F, t241: F, t136: F, t41667: F, t41671: F, t908: F, t10319: F, t699: F, t10313: F, t2826: F, t41649: F, t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F) -> (F, F, F, F, F, F) {
    let t41880 = t241 * t10213;
    let t41882 = t136 * t41880 * t41667;
    let t41885 = t136 * t908 * t41671;
    let t41887 = t699 * t10319;
    let t41889 = t699 * t10313;
    let t41892 = t136 * t2826 * t41649;
    let t41904 = 280.0 / 81.0 * t41654;
    let t41912 = 2.0 * t41642 + 8.0 / 3.0 * t41646 + 8.0 * t41651 + t41904 - 8.0 / 9.0 * t41656 - 16.0 / 27.0 * t41658 + 40.0 / 81.0 * t41660 + 4.0 / 9.0 * t41662 - 80.0 / 81.0 * t41669 - t41673 / 3.0 + 16.0 / 9.0 * t41675;
    (t41882, t41885, t41887, t41889, t41892, t41912)
}
