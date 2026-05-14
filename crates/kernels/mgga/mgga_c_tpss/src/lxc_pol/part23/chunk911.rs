//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 911/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk911<F: Float>(t2685: F, t2689: F, t672: F, t930: F, t925: F, t2748: F, t2753: F, t361: F, t650: F, t242: F, t949: F, t946: F, t2469: F, t2751: F, t967: F, t140: F, t2699: F) -> (F, F, F, F, F, F, F) {
    let t8453 = t2685 * t2689;
    let t8455 = t672 * t930;
    let t8456 = t925 * t8455;
    let t8462 = t2748 * t2753;
    let t8469 = t650 * t361;
    let t8471 = t242 * t8469 * t949;
    let t8472 = t946 * t8471;
    let t8480 = t242 * t2751 * t2469;
    let t8481 = t967 * t8480;
    let t8483 = t140 * t2699;
    (t8453, t8456, t8462, t8469, t8472, t8481, t8483)
}
