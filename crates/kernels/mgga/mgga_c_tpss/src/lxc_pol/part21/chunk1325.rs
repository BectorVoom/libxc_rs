//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1325/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1325<F: Float>(t1678: F, t65157: F, t19407: F, t619: F, t77: F, t1679: F, t3486: F, t1290: F, t7682: F, t1981: F, t3426: F, t3432: F, t7690: F, t1982: F, t6076: F, t18345: F, t18350: F, t18352: F, t19346: F, t19349: F, t61939: F, t62010: F, t62021: F, t62024: F, t62033: F, t65152: F) -> (F,) {
    let t65158 = t1678 * t65157;
    let t65162 = t77 * t19407 * t619;
    let t65165 = t1679 * t3486;
    let t65166 = t1678 * t65165;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    let t65178 = t7690 * t1290;
    let t65182 = t77 * t6076 * t1982;
    let t65185 = -10.0 / 3.0 * t19349 * t62010 - 10.0 / 3.0 * t19349 * t61939 - 5.0 * t18345 * t65152 - 5.0 / 3.0 * t62024 * t19346 - 10.0 / 3.0 * t18350 * t65158 - 10.0 * t18345 * t65162 - 10.0 / 3.0 * t18350 * t65166 - 10.0 / 3.0 * t65169 * t18352 - 10.0 / 3.0 * t65172 * t18352 - 10.0 / 3.0 * t65175 * t18352 + 10.0 * t65178 * t62021 + 35.0 * t62033 * t65182;
    (t65185,)
}
