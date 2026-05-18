//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 724/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk724<F: Float>(t1369: F, t2143: F, t1368: F, t750: F, t762: F, t124: F, t3610: F, t2158: F, t236: F, t339: F, t238: F, t72: F) -> (F, F, F, F, F, F) {
    let t3615 = t2143 * t1369;
    let t3618 = t762 * t1368 * t750;
    let t3621 = t124 * t3610;
    let t3622 = t762 * t3621;
    let t3626 = t339 * t2158 * t236;
    let t3627 = t238 * t72;
    (t3615, t3618, t3621, t3622, t3626, t3627)
}
