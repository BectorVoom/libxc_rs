//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1081/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1081<F: Float>(t14459: F, t4496: F, t959: F, t17194: F, t17197: F, t17209: F, t17301: F, t17303: F, t17306: F, t17372: F, t17374: F, t17377: F, t17379: F, t17425: F, t17427: F, t17561: F, t17563: F, t17568: F, t17929: F) -> (F, F) {
    let t17930 = t4496 * t14459;
    let t17932 = F::cast_from(0.34631718211362927518e2_f64) * t959 * t17930;
    let t17933 = t17194 + t17197 - t17209 - t17301 - t17303 - t17306 + t17561 - t17563 - t17568 + t17372 + t17374 - t17377 + t17379 + t17425 + t17427 + t17929 - t17932;
    (t17932, t17933)
}
