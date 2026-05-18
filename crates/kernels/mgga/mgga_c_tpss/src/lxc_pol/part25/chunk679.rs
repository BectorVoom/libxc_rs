//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 679/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk679<F: Float>(t33: F, t259: F, t479: F, t3735: F, t4332: F, t1006: F, t1157: F, t1289: F, t1402: F, t1497: F, t1594: F, t3431: F, t3743: F, t481: F, t57: F, t581: F, t826: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t4333 = piecewise3::<f64>(t480, t4332, t3735);
    let t4340 = piecewise3::<f64>(t386, t3735 * t33 / F::new(2.0) + t1402 * t1006 / F::new(2.0) + t826 * t1497 / F::new(2.0) - t3743, -t1157 * t1289 / F::new(2.0) - t1594 * t581 / F::new(2.0) - t481 * t3431 / F::new(2.0) + t4333 * t57 / F::new(2.0));
    (t4333, t4340)
}
