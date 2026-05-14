//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 866/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk866<F: Float>(t1041: F, t10454: F, t121: F, t3061: F, t248: F, t2771: F, t10321: F, t1044: F, t1008: F) -> (F, F, F, F, F) {
    let t10455 = t1041 * t10454;
    let t10457 = t121 * t3061;
    let t10459 = t248 * t10457 * t2771;
    let t10460 = t1041 * t10459;
    let t10463 = t248 * t1044 * t10321;
    let t10468 = t1008 * t1008;
    let t10469 = 1.0 / t10468;
    (t10455, t10459, t10460, t10463, t10469)
}
