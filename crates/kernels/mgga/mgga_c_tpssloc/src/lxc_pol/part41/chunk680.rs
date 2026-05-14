//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 680/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk680<F: Float>(t25: F, t265: F, t394: F, t4324: F, t4704: F, t1074: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t3966: F, t40: F, t4332: F, t606: F, t607: F, t873: F, t1654: F, t690: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t4705 = piecewise3(t395, t4704, t4324);
    let t4712 = piecewise3(t115, t4324 * t25 / 2.0 + t1534 * t606 / 2.0 + t873 * t1408 / 2.0 + t4332, t1074 * t1409 / 2.0 + t1642 * t607 / 2.0 + t396 * t3966 / 2.0 + t4705 * t40 / 2.0);
    let t4721 = t690 * t1654;
    (t4705, t4712, t4721)
}
