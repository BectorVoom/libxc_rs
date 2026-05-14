//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1216/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1216<F: Float>(t20509: F, t2436: F, t6353: F, t8096: F, t1692: F, t17938: F, t18053: F, t18056: F, t18059: F, t1812: F, t18728: F, t18807: F, t19678: F, t19821: F, t20514: F, t20526: F, t2439: F, t5591: F, t5853: F, t62610: F, t6354: F, t63771: F, t63791: F, t63806: F, t63837: F, t63845: F, t64256: F, t64267: F, t64297: F) -> (F, F, F) {
    let t66281 = t20509 * t2436;
    let t66299 = t6353 * t8096;
    let t66302 = 2.0 * t20526 * t64267 - 3.0 / 2.0 * t18728 * t64297 + 3.0 / 2.0 * t2439 * t6354 * t17938 - t1692 * t18807 * t19821 + 3.0 / 2.0 * t2439 * t1812 * t63806 - t1692 * t20514 * t18056 - t1692 * t66281 * t5591 - t1692 * t5853 * t63791 - 3.0 * t62610 * t19678 + t20526 * t63845 + 2.0 * t20526 * t63837 - t1692 * t20514 * t18059 / 2.0 - 3.0 / 2.0 * t18728 * t64256 - t1692 * t5853 * t63771 / 2.0 + t1692 * t66299 * t18053;
    (t66281, t66299, t66302)
}
