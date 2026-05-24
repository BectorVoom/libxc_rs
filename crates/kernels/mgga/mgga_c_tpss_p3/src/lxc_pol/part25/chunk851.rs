//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 851/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk851<F: Float>(t33: F, t259: F, t479: F, t1812: F, t6207: F, t6373: F, t1289: F, t1497: F, t1692: F, t1826: F, t2439: F, t57: F, t5853: F, t6214: F, t6354: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t6380 = t1812 * t6207;
    let t6393 = piecewise3::<F>(t480, F::new(0.0), t6373);
    let t6398 = piecewise3::<F>(t386, F::new(3.0) / F::new(2.0) * t2439 * t6380 + t1692 * t6354 * t33 / F::new(2.0) - t1692 * t5853 * t6214 / F::new(2.0) + t1692 * t1812 * t1497 / F::new(2.0), -t1826 * t1289 / F::new(2.0) + t6393 * t57 / F::new(2.0));
    (t6393, t6398)
}
