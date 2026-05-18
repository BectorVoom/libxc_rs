//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 958/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk958<F: Float>(t20936: F, t225: F, t237: F, t119: F, t20756: F, t210: F, t1484: F, t5544: F) -> (F, F, F, F) {
    let t20937 = t20936 * t225;
    let t20938 = t20937 * t237;
    let t20943 = t119 * t20756;
    let t20944 = t210 * t20943;
    let t20947 = t1484 * t5544;
    (t20937, t20938, t20944, t20947)
}
