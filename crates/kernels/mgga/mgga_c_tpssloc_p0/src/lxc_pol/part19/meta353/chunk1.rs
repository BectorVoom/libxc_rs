//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1281/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281<F: Float>(t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41904: F, t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F) -> (F, F) {
    let t41912 = F::new(2.0) * t41642 + F::new(8.0) / F::new(3.0) * t41646 + F::new(8.0) * t41651 + t41904 - F::new(8.0) / F::new(9.0) * t41656 - F::new(16.0) / F::new(27.0) * t41658 + F::new(40.0) / F::new(81.0) * t41660 + F::new(4.0) / F::new(9.0) * t41662 - F::new(80.0) / F::new(81.0) * t41669 - t41673 / F::new(3.0) + F::new(16.0) / F::new(9.0) * t41675;
    let t41925 = -F::new(16.0) / F::new(9.0) * t41678 + F::new(8.0) / F::new(9.0) * t41680 + F::new(8.0) / F::new(3.0) * t41682 + F::new(112.0) / F::new(81.0) * t41684 + F::new(40.0) / F::new(9.0) * t41690 - F::new(20.0) / F::new(9.0) * t41695 - F::new(8.0) * t41699 - F::new(2.0) / F::new(3.0) * t41703 - F::new(8.0) / F::new(9.0) * t41707 + F::new(8.0) * t41711 - F::new(8.0) / F::new(3.0) * t41713 - F::new(12.0) * t41717;
    (t41912, t41925)
}
