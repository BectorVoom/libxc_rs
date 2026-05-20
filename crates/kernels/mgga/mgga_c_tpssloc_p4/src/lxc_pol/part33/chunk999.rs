//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 999/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk999<F: Float>(t10482: F, t21390: F, t1021: F, t248: F, t3131: F, t360: F, t10278: F, t20234: F, t2979: F, t21122: F, t4510: F, t13769: F, t17863: F) -> (F, F, F, F, F, F) {
    let t21391 = t21390 * t10482;
    let t21393 = t248 * t1021 * t21391;
    let t21396 = t21390 * t3131;
    let t21398 = t248 * t1021 * t21396;
    let t21403 = t21390 * t360;
    let t21405 = t248 * t1021 * t21403;
    let t21409 = t10278 * t20234;
    let t21410 = t2979 * t21409;
    let t21413 = t4510 * t21122;
    let t21416 = t13769 * t17863;
    (t21393, t21398, t21405, t21410, t21413, t21416)
}
