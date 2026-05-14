//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 741/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk741<F: Float>(t3721: F, t812: F, t1396: F, t2401: F, t253: F, t3693: F, t3695: F, t3699: F, t809: F, t819: F) -> (F, F) {
    let t3722 = t812 * t3721;
    let t3724 = -t1396 * t2401 + t253 * t3693 - t3695 * t819 + 2.0 * t3699 * t809 - t3722 * t809;
    (t3722, t3724)
}
