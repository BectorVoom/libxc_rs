//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 592/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk592<F: Float>(t5705: F, t894: F, t2815: F, t5698: F, t901: F, t2826: F, t5677: F, t136: F, t5681: F, t908: F, t5685: F, t2810: F, t2823: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5706 = t894 * t5705;
    let t5712 = t2815 * t5698;
    let t5714 = t901 * t5705;
    let t5717 = t2826 * t5677;
    let t5718 = t136 * t5717;
    let t5720 = t908 * t5681;
    let t5721 = t136 * t5720;
    let t5723 = t908 * t5685;
    let t5724 = t136 * t5723;
    let t5726 = -0.9494625e0 * t5699 + 0.1898925e1 * t5706 + t2810 + 0.19931111111111111111e0 * t4335 - 0.19931111111111111111e0 * t5679 + 0.59793333333333333334e0 * t5683 - 0.29896666666666666667e0 * t5687 + 0.15358125e0 * t5712 + 0.3071625e0 * t5714 + t2823 + 0.10954222222222222222e0 * t4384 - 0.27385555555555555556e-1 * t5718 + 0.16431333333333333333e0 * t5721 - 0.82156666666666666667e-1 * t5724;
    (t5706, t5712, t5714, t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
