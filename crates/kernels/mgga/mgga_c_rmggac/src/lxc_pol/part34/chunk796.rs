//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 796/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk796<F: Float>(t73875: F, t73877: F, t73879: F, t73857: F, t73862: F, t76713: F, t76718: F, t76723: F, t76728: F, t76733: F, t76738: F, t76743: F, t76744: F, t76745: F, t76748: F, t76749: F, t76750: F) -> (F,) {
    let t76751 = 0.2553875993597870364e-4 * t73875;
    let t76752 = 0.2553875993597870364e-4 * t73877;
    let t76753 = 0.1702583995731913576e-4 * t73879;
    let t76754 = -t76713 + t76718 - t76723 + t76728 - t76733 - t76738 + t76743 + t76744 - t76745 - 0.87596530464506835935e-6 * t73857 + 0.87596530464506835935e-6 * t73862 - t76748 - t76749 + t76750 - t76751 + t76752 + t76753;
    (t76754,)
}
