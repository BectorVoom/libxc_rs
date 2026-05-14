//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1009/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1009<F: Float>(t1164: F, t18283: F, t1190: F, t6238: F, t1743: F, t4965: F, t486: F, t6224: F, t11721: F, t1215: F, t4582: F, t4978: F, t1222: F, t6170: F, t6158: F, t6165: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18285 = 0.34631718211362927518e2 * t1164 * t18283;
    let t18287 = t1190 * t6238;
    let t18297 = t4965 * t1743;
    let t18300 = t486 * t6224;
    let t18301 = t11721 * t1215;
    let t18302 = t18300 * t18301;
    let t18303 = t4582 * t18302;
    let t18306 = t18300 * t4978;
    let t18307 = t4582 * t18306;
    let t18310 = t6170 * t1222;
    let t18312 = t6158 * t1222;
    let t18314 = t6165 * t1222;
    (t18285, t18287, t18297, t18300, t18303, t18307, t18310, t18312, t18314)
}
