//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1036/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1036<F: Float>(t10416: F, t4278: F, t3931: F, t10412: F, t3096: F, t9199: F, t11476: F, t4231: F, t9721: F, t3053: F, t9619: F, t3055: F, t9187: F, t9684: F, t4283: F, t10353: F, t1101: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12482 = t4278 * t10416;
    let t12483 = t3931 * t12482;
    let t12486 = t4278 * t10412;
    let t12487 = t3931 * t12486;
    let t12490 = t3096 * t9199;
    let t12491 = t12490 * t11476;
    let t12492 = t3931 * t12491;
    let t12497 = t4231 * t9721;
    let t12498 = t3931 * t12497;
    let t12501 = t9619 * t3053;
    let t12502 = t4231 * t12501;
    let t12503 = t3931 * t12502;
    let t12506 = t4231 * t3055;
    let t12507 = t3931 * t12506;
    let t12510 = t9684 * t9187;
    let t12511 = t12510 * t11476;
    let t12512 = t3931 * t12511;
    let t12515 = t4283 * t10416;
    let t12516 = t3931 * t12515;
    let t12519 = t4283 * t10412;
    let t12520 = t3931 * t12519;
    let t12523 = t1101 * t10353;
    (t12483, t12487, t12492, t12498, t12503, t12507, t12512, t12516, t12520, t12523)
}
