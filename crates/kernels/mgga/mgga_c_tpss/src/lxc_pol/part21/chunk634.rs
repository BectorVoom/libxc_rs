//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 634/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk634<F: Float>(t2688: F, t2767: F, t219: F, t976: F, t371: F, t979: F) -> (F, F, F, F) {
    let t2768 = t2688 + t2767;
    let t2769 = param_beta * t2768;
    let t2771 = t976 * t219;
    let t2775 = 1.0 / t979 / t371;
    (t2768, t2769, t2771, t2775)
}
