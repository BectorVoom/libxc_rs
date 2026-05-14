//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1056/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1056<F: Float>(t5493: F, t89: F, t1874: F, t7458: F, t7461: F, t4028: F, t7468: F, t28002: F, t19451: F, t1774: F, t7467: F, t652: F, t2006: F, t6361: F, t1807: F, t7722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28030 = t89 * t5493;
    let t28032 = 2.0 * t28030 * t1874;
    let t28034 = 4.0 * t7458 * t7461;
    let t28036 = 4.0 * t4028 * t7468;
    let t28038 = 4.0 * t28002 * t1874;
    let t28040 = 4.0 * t4028 * t7461;
    let t28042 = 2.0 * t19451 * t1874;
    let t28045 = t1774 * t7467;
    let t28047 = 4.0 * t652 * t28045;
    let t28051 = t6361 * t2006;
    let t28053 = t1807 * t7722;
    (t28030, t28032, t28034, t28036, t28038, t28040, t28042, t28045, t28047, t28051, t28053)
}
