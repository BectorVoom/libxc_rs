//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 617/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk617<F: Float>(t2826: F, t5677: F, t136: F, t5681: F, t908: F, t5685: F, t2810: F, t2823: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F, t5706: F, t5712: F, t5714: F) -> (F, F, F, F, F, F, F) {
    let t5717 = t2826 * t5677;
    let t5718 = t136 * t5717;
    let t5720 = t908 * t5681;
    let t5721 = t136 * t5720;
    let t5723 = t908 * t5685;
    let t5724 = t136 * t5723;
    let t5726 = -F::new(0.9494625e0) * t5699 + F::new(0.1898925e1) * t5706 + t2810 + F::cast_from(0.19931111111111111111e0_f64) * t4335 - F::cast_from(0.19931111111111111111e0_f64) * t5679 + F::cast_from(0.59793333333333333334e0_f64) * t5683 - F::cast_from(0.29896666666666666667e0_f64) * t5687 + F::new(0.15358125e0) * t5712 + F::new(0.3071625e0) * t5714 + t2823 + F::cast_from(0.10954222222222222222e0_f64) * t4384 - F::cast_from(0.27385555555555555556e-1_f64) * t5718 + F::cast_from(0.16431333333333333333e0_f64) * t5721 - F::cast_from(0.82156666666666666667e-1_f64) * t5724;
    (t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
