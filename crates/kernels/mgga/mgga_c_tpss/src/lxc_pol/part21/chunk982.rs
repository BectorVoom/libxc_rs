//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 982/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk982<F: Float>(t10575: F, t10581: F, t10587: F, t10592: F, t10596: F, t10600: F, t10602: F, t10606: F, t10610: F, t10614: F, t10617: F, t10620: F, t2173: F, t3626: F, t10552: F, t774: F, t801: F) -> (F, F) {
    let t10621 = -5.0 / 384.0 * t2173 * t10575 + t2173 * t10581 / 384.0 - t3626 * t10587 / 192.0 - t2173 * t10592 / 1536.0 - t2173 * t10596 / 3072.0 + t10600 + t2173 * t10602 / 384.0 + t2173 * t10606 / 768.0 + t3626 * t10610 / 768.0 + t3626 * t10614 / 1536.0 - 119.0 / 3456.0 * t10617 + t10620;
    let t10623 = t801 * t774 * t10552;
    (t10621, t10623)
}
