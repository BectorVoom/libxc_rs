//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 427/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk427<F: Float>(t30: F, t1288: F, t1289: F, t1402: F, t1490: F, t259: F, t381: F, t45: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t1495 = piecewise3::<F>(t120, t259 * t1288 / F::new(2.0) + t1402 * t30 / F::new(2.0), t381 * t1289 / F::new(2.0) + t1490 * t45 / F::new(2.0));
    let t1497 = -t1288;
    (t1495, t1497)
}
