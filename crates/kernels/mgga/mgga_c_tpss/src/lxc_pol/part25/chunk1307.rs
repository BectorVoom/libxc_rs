//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1307/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1307<F: Float>(t38: F, t42690: F, t13442: F, t76: F, t4622: F, t619: F, t77: F, t13546: F, t94: F, t13866: F, t1705: F, t935: F) -> (F, F, F, F, F) {
    let t69281 = t42690 * t38;
    let t69338 = t76 * t13442;
    let t69355 = t77 * t4622 * t619;
    let t69383 = t94 * t13546;
    let t69452 = t1705 * t13866 * t935;
    (t69281, t69338, t69355, t69383, t69452)
}
