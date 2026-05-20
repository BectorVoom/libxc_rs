//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2385/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385<F: Float>(t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t41904: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F) -> F {
    let t48980 = F::new(40.0) / F::new(9.0) * t47744 + F::new(8.0) * t47748 - F::new(4.0) / F::new(9.0) * t41656 - F::new(8.0) / F::new(27.0) * t41658 + F::new(10.0) / F::new(81.0) * t41660 + t41662 / F::new(9.0) + F::new(8.0) / F::new(9.0) * t41675 - F::new(4.0) / F::new(9.0) * t41678 + F::new(2.0) / F::new(3.0) * t41682 + F::new(28.0) / F::new(27.0) * t41684 + F::new(2.0) * t47761 + F::new(2.0) * t47765 + F::new(2.0) / F::new(3.0) * t47769 + F::new(2.0) / F::new(9.0) * t41680 - F::new(2.0) / F::new(3.0) * t41713 + F::new(4.0) * t47777 + t41904 - F::new(10.0) / F::new(9.0) * t47781 - F::new(6.0) * t47785 + F::new(28.0) / F::new(81.0) * t47787;
    t48980
}
