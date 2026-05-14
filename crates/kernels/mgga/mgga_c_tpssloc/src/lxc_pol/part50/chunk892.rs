//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 892/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk892<F: Float>(t25: F, t265: F, t394: F, t1068: F, t1070: F, t1637: F, t193: F, t23738: F, t23742: F, t25836: F, t25840: F, t25845: F, t25882: F, t336: F, t4696: F, t4700: F, t6822: F, t1409: F, t1965: F, t25398: F, t3966: F, t40: F, t607: F, t6835: F, t7643: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t25883 = piecewise3(t395, t1070 * t193 * t25836 * t336 - t1068 * t25840 * t4700 - t1637 * t23738 * t4700 + 2.0 * t23742 * t25845 * t4700 - t4696 * t4700 * t6822, t25882);
    let t25890 = piecewise3(t115, t25398, t6835 * t1409 / 2.0 + t1965 * t3966 / 2.0 + t25883 * t40 / 2.0 + t7643 * t607 / 2.0);
    (t25890,)
}
