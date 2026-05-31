//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2335/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2335<F: Float>(t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t41741: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F) -> F {
    let t47789 = F::cast_from(0.12361111111111111111e0_f64) * t47744 + F::cast_from(0.2225e0_f64) * t47748 - F::cast_from(0.12361111111111111111e-1_f64) * t41656 - F::cast_from(0.82407407407407407408e-2_f64) * t41658 + F::cast_from(0.34336419753086419753e-2_f64) * t41660 + F::cast_from(0.30902777777777777778e-2_f64) * t41662 + F::cast_from(0.24722222222222222222e-1_f64) * t41675 - F::cast_from(0.12361111111111111111e-1_f64) * t41678 + F::cast_from(0.18541666666666666667e-1_f64) * t41682 + F::cast_from(0.28842592592592592593e-1_f64) * t41684 + F::cast_from(0.55625000000000000001e-1_f64) * t47761 + F::cast_from(0.55625000000000000001e-1_f64) * t47765 + F::cast_from(0.18541666666666666667e-1_f64) * t47769 + F::cast_from(0.61805555555555555556e-2_f64) * t41680 - F::cast_from(0.18541666666666666667e-1_f64) * t41713 + F::cast_from(0.11125e0_f64) * t47777 + t41741 - F::cast_from(0.30902777777777777778e-1_f64) * t47781 - F::cast_from(0.166875e0_f64) * t47785 + F::cast_from(0.96141975308641975309e-2_f64) * t47787;
    t47789
}
