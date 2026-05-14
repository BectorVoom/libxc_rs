//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1339/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1339<F: Float>(t68846: F, t68910: F, t68957: F, t69002: F, t69386: F, t69423: F, t69788: F, t71019: F, t1673: F, t6279: F, t21572: F, t546: F, t1668: F, t20119: F, t13546: F, t547: F, t5772: F) -> (F, F, F, F, F) {
    let t71022 = t68846 + t68910 + t68957 + t69002 + t69386 + t69423 + t69788 + t71019;
    let t71025 = t6279 * t1673;
    let t71030 = t546 * t21572;
    let t71032 = 12.0 * t1668 * t20119;
    let t71037 = 6.0 * t547 * t5772 * t13546;
    (t71022, t71025, t71030, t71032, t71037)
}
