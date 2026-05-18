//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1193/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1193<F: Float>(t226: F, t782: F, t818: F, t2157: F, t811: F, t2433: F, t30: F, t580: F, t821: F, t2428: F, t2116: F, t33: F) -> (F, F, F, F, F, F) {
    let t18009 = t818 * t782 * t226;
    let t18021 = t811 * t2157;
    let t18053 = t30 * t2433;
    let t18056 = t580 * t821;
    let t18059 = t30 * t2428;
    let t18239 = t33 * t2116;
    (t18009, t18021, t18053, t18056, t18059, t18239)
}
