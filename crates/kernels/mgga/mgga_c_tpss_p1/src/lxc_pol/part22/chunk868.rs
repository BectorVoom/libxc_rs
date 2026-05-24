//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 868/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk868<F: Float>(t30: F, t259: F, t379: F, t1364: F, t1812: F, t207: F, t6353: F, t1398: F, t1692: F, t198: F, t2439: F, t5853: F, t823: F, t1288: F, t1289: F, t1819: F, t45: F, t6153: F, t6331: F, t6354: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t6365 = t1812 * t1364;
    let t6368 = t207 * t6353;
    let t6373 = -t1398 * t1692 * t5853 + t198 * t6368 * t823 + F::new(3.0) * t2439 * t6365;
    let t6374 = piecewise3::<F>(t380, F::new(0.0), t6373);
    let t6379 = piecewise3::<F>(t120, F::new(3.0) / F::new(2.0) * t2439 * t6331 + t1692 * t6354 * t30 / F::new(2.0) - t1692 * t5853 * t6153 / F::new(2.0) + t1692 * t1812 * t1288 / F::new(2.0), t1819 * t1289 / F::new(2.0) + t6374 * t45 / F::new(2.0));
    (t6365, t6373, t6374, t6379)
}
