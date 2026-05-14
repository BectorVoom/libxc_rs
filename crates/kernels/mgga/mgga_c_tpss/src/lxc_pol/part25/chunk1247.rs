//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1247/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1247<F: Float>(t21658: F, t2436: F, t1288: F, t13334: F, t1692: F, t1812: F, t18728: F, t19678: F, t19810: F, t20417: F, t20510: F, t20526: F, t21583: F, t2439: F, t36547: F, t5591: F, t6153: F, t66281: F, t66317: F, t69796: F, t69817: F, t69820: F, t69858: F, t69882: F, t69887: F, t70221: F) -> (F, F) {
    let t72265 = t21658 * t2436;
    let t72277 = 3.0 / 2.0 * t2439 * t1812 * t69887 - 3.0 * t66317 * t19678 - t1692 * t66281 * t6153 - 3.0 * t66317 * t19810 + 3.0 * t36547 * t21583 - 3.0 * t20417 * t69820 + t1692 * t20510 * t1288 + t1692 * t1812 * t13334 / 2.0 + 3.0 / 2.0 * t2439 * t1812 * t70221 - t1692 * t72265 * t5591 / 2.0 + 6.0 * t20417 * t69817 - 3.0 * t18728 * t69796 + 2.0 * t20526 * t69882 - 3.0 / 2.0 * t18728 * t69858;
    (t72265, t72277)
}
