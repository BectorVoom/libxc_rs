//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 828/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk828<F: Float>(t14385: F, t39277: F, t2144: F, t2447: F, t507: F, t2136: F, t235: F, t7190: F, t2141: F, t7262: F, t2147: F, t74943: F, t74948: F, t14710: F, t2868: F, t739: F, t7799: F, t9530: F) -> (F, F, F, F, F, F, F, F) {
    let t77264 = t39277 * t14385;
    let t77265 = 0.53205749866622299248e-5 * t77264;
    let t77269 = t507 * t2144 * t2447;
    let t77270 = t77269 * t2136;
    let t77271 = 0.10227998120342003148e-1 * t77270;
    let t77273 = t235 * t7190 * t2447;
    let t77274 = t77273 * t2141;
    let t77275 = 0.13637330827122670864e-1 * t77274;
    let t77277 = t235 * t7262 * t2447;
    let t77278 = t77277 * t2147;
    let t77279 = 0.68186654135613354322e-2 * t77278;
    let t77280 = 0.2553875993597870364e-4 * t74943;
    let t77281 = 0.2553875993597870364e-4 * t74948;
    let t77282 = t2868 * t14710;
    let t77283 = 0.2993560425465952141e-1 * t77282;
    let t77286 = 0.11974241701863808564e0 * t739 * t9530 * t7799;
    (t77265, t77271, t77275, t77279, t77280, t77281, t77283, t77286)
}
