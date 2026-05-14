//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 761/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk761<F: Float>(t30: F, t259: F, t379: F, t4818: F, t5047: F, t1288: F, t1289: F, t1402: F, t1490: F, t381: F, t45: F, t4578: F, t4579: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t5048 = piecewise3(t380, t5047, t4818);
    let t5055 = piecewise3(t120, t4818 * t30 / 2.0 + t1402 * t1288 + t259 * t4578 / 2.0, t5048 * t45 / 2.0 + t1490 * t1289 + t381 * t4579 / 2.0);
    let t5059 = -t4578;
    (t5048, t5055, t5059)
}
