//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 645/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk645<F: Float>(t182: F, t6320: F, t2408: F, t2417: F, t2423: F, t2426: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t6299: F, t6300: F, t6301: F, t6304: F) -> (F, F) {
    let t6322 = 0.19751673498613801407e-1 * t6320 * t182;
    let t6323 = 6.0 * t3918 * t6301 + t2408 + t2417 - t2423 - t2426 + t3686 + t3688 - t3690 - t3695 + t3813 - t6299 - t6300 + t6304 + t6322;
    (t6322, t6323)
}
