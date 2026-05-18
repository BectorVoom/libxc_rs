//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1043/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1043<F: Float>(t14423: F, t812: F, t10821: F, t1396: F, t14350: F, t14352: F, t14363: F, t14367: F, t14372: F, t2401: F, t253: F, t3695: F, t3699: F, t3722: F, t4784: F, t4800: F, t809: F, t819: F) -> (F, F) {
    let t14424 = t812 * t14423;
    let t14426 = -F::new(2.0) * t10821 * t1396 + t14350 * t253 - t14352 * t819 - F::new(6.0) * t14363 * t809 + F::new(4.0) * t14367 * t809 + F::new(2.0) * t14372 * t809 - t14424 * t809 + F::new(2.0) * t2401 * t4784 - t2401 * t4800 + F::new(4.0) * t3695 * t3699 - F::new(2.0) * t3695 * t3722;
    (t14424, t14426)
}
