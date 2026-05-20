//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2223/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2223<F: Float>(t17191: F, t942: F, t2929: F, t5769: F, t2791: F, t5689: F, t2885: F, t5737: F, t2904: F, t10632: F, t5790: F, t17422: F, t2844: F) -> (F, F, F, F, F, F, F) {
    let t60338 = t17191 * t942;
    let t60343 = t5769 * t2929;
    let t60357 = t5689 * t2791;
    let t60407 = t5737 * t2885;
    let t60424 = t5769 * t2904;
    let t60722 = t5790 * t10632;
    let t60745 = t17422 * t2844;
    (t60338, t60343, t60357, t60407, t60424, t60722, t60745)
}
