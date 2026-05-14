//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1093/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1093<F: Float>(t25: F, t265: F, t394: F, t1070: F, t1637: F, t193: F, t23742: F, t25840: F, t28719: F, t28755: F, t336: F, t4700: F, t5946: F, t5950: F, t6822: F, t1409: F, t1965: F, t28469: F, t40: F, t5398: F, t7643: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t28756 = piecewise3(t395, t1070 * t193 * t28719 * t336 - 2.0 * t1637 * t25840 * t4700 + 2.0 * t23742 * t4700 * t5950 - t4700 * t5946 * t6822, t28755);
    let t28763 = piecewise3(t115, t28469, t28756 * t40 / 2.0 + t7643 * t1409 + t1965 * t5398 / 2.0);
    (t28756, t28763)
}
