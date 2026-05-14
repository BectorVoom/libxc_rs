//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1291/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1291<F: Float>(t19602: F, t5706: F, t2049: F, t6076: F, t77: F, t1317: F, t5506: F, t19407: F, t619: F, t1679: F, t3486: F, t1290: F, t7682: F, t1981: F, t3426: F, t3432: F) -> (F, F, F, F, F, F, F, F) {
    let t65143 = 2.0 * t5706 * t19602;
    let t65152 = t77 * t6076 * t2049;
    let t65157 = t5506 * t1317;
    let t65162 = t77 * t19407 * t619;
    let t65165 = t1679 * t3486;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    (t65143, t65152, t65157, t65162, t65165, t65169, t65172, t65175)
}
