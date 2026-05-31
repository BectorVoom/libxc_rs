//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1129/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1129<F: Float>(t12506: F, t3931: F, t9187: F, t9684: F, t11476: F, t10416: F, t4283: F, t10412: F, t10353: F, t1101: F, t926: F, t3028: F, t4212: F) -> (F, F, F, F, F, F) {
    let t12507 = t3931 * t12506;
    let t12510 = t9684 * t9187;
    let t12511 = t12510 * t11476;
    let t12512 = t3931 * t12511;
    let t12515 = t4283 * t10416;
    let t12516 = t3931 * t12515;
    let t12519 = t4283 * t10412;
    let t12520 = t3931 * t12519;
    let t12523 = t1101 * t10353;
    let t12524 = t926 * t12523;
    let t12530 = t4212 * t3028 / F::cast_from(162.0_f64);
    (t12507, t12512, t12516, t12520, t12524, t12530)
}
