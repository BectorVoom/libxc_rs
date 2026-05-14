//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1177/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1177<F: Float>(t29895: F, t30060: F, t29900: F, t30067: F, t111: F, t8199: F, t112: F, t30094: F, t1404: F, t656: F, t9576: F, t2331: F, t2585: F, t2: F, t666: F, t1851: F, t8217: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110338 = t29895 * t30060;
    let t110340 = t29900 * t30067;
    let t110363 = t8199 * t111;
    let t110376 = t30094 * t112;
    let t110484 = t8199 * t1404;
    let t110532 = t9576 * t656;
    let t110601 = t2585 * t2331;
    let t110602 = t2 * t666;
    let t110919 = 2.0 * t1851 * t8217;
    (t110338, t110340, t110363, t110376, t110484, t110532, t110601, t110602, t110919)
}
