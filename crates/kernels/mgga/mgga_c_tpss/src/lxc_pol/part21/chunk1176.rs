//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1176/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1176<F: Float>(t10514: F, t18246: F, t1006: F, t750: F, t2133: F, t33: F, t2433: F, t821: F, t2428: F, t1692: F, t1713: F, t17929: F, t18043: F, t18047: F, t18052: F, t18239: F, t2439: F, t2829: F, t3552: F, t5586: F, t5590: F, t5671: F, t5678: F) -> (F, F, F, F, F, F, F) {
    let t18247 = t18246 * t10514;
    let t18250 = t1006 * t750;
    let t18254 = t33 * t2133;
    let t18265 = t33 * t2433;
    let t18268 = t1006 * t821;
    let t18271 = t33 * t2428;
    let t18278 = 3.0 * t3552 * t1713 * t18239 + 3.0 * t2439 * t5586 * t5671 - 3.0 * t17929 * t18247 + 3.0 * t2439 * t1713 * t18250 + 3.0 / 2.0 * t2439 * t1713 * t18254 + t1692 * t18043 * t33 / 2.0 - t1692 * t18047 * t5678 + t1692 * t5586 * t1006 + t1692 * t18052 * t18265 - t1692 * t5590 * t18268 - t1692 * t5590 * t18271 / 2.0 + t1692 * t1713 * t2829 / 2.0;
    (t18247, t18250, t18254, t18265, t18268, t18271, t18278)
}
