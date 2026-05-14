//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 673/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk673<F: Float>(t30: F, t259: F, t379: F, t3735: F, t4027: F, t1288: F, t1289: F, t1402: F, t1490: F, t3431: F, t3743: F, t381: F, t45: F, t580: F, t581: F, t826: F, t999: F, t1502: F, t664: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t4028 = piecewise3(t380, t4027, t3735);
    let t4035 = piecewise3(t120, t3735 * t30 / 2.0 + t1402 * t580 / 2.0 + t826 * t1288 / 2.0 + t3743, t999 * t1289 / 2.0 + t1490 * t581 / 2.0 + t381 * t3431 / 2.0 + t4028 * t45 / 2.0);
    let t4044 = t664 * t1502;
    (t4028, t4035, t4044)
}
