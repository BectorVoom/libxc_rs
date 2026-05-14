//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 487/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk487<F: Float>(t241: F, t976: F, t2771: F, t136: F, t2776: F, t908: F, t2780: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2810: F, t2816: F, t2818: F, t2823: F, t2824: F) -> (F, F, F, F, F, F, F, F) {
    let t2826 = t241 * t976;
    let t2827 = t2826 * t2771;
    let t2828 = t136 * t2827;
    let t2830 = t908 * t2776;
    let t2831 = t136 * t2830;
    let t2833 = t908 * t2780;
    let t2834 = t136 * t2833;
    let t2836 = -0.9494625e0 * t2800 + 0.1898925e1 * t2808 + t2810 + 0.19931111111111111111e0 * t2766 - 0.19931111111111111111e0 * t2773 + 0.59793333333333333334e0 * t2778 - 0.29896666666666666667e0 * t2782 + 0.15358125e0 * t2816 + 0.3071625e0 * t2818 + t2823 + 0.10954222222222222222e0 * t2824 - 0.27385555555555555556e-1 * t2828 + 0.16431333333333333333e0 * t2831 - 0.82156666666666666667e-1 * t2834;
    (t2826, t2827, t2828, t2830, t2831, t2833, t2834, t2836)
}
