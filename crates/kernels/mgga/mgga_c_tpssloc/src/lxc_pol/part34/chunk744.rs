//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 744/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk744<F: Float>(t4993: F, t5005: F, t5024: F, t1017: F, t6163: F, t1210: F, t1207: F, t372: F, t479: F, t471: F, t248: F, t3521: F, t5979: F, t1227: F, t1009: F, t6150: F) -> (F, F, F, F, F, F) {
    let t18980 = t5005 * t4993;
    let t18987 = t5024 * t4993;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19031 = t6163 * t372;
    let t19032 = t479 * t19031;
    let t19033 = t471 * t19032;
    let t19040 = t248 * t3521 * t5979;
    let t19041 = t1227 * t19040;
    let t19045 = t6150 * t1009;
    (t18980, t18987, t19026, t19033, t19041, t19045)
}
