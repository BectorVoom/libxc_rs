//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1460/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1460<F: Float>(t372: F, t6163: F, t479: F, t471: F, t248: F, t3521: F, t5979: F, t1227: F, t1009: F, t6150: F, t1011: F, t1212: F) -> (F, F, F, F) {
    let t19031 = t6163 * t372;
    let t19032 = t479 * t19031;
    let t19033 = t471 * t19032;
    let t19040 = t248 * t3521 * t5979;
    let t19041 = t1227 * t19040;
    let t19045 = t6150 * t1009;
    let t19046 = t19045 * t1011;
    let t19047 = t19046 * t1212;
    (t19033, t19041, t19045, t19047)
}
