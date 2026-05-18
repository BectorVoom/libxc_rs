//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 306/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk306<F: Float>(t30: F, t259: F, t379: F, t198: F, t330: F, t826: F, t843: F, t869: F, t909: F, t911: F, t916: F, t993: F, t995: F, t381: F, t45: F, t580: F, t581: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t999 = piecewise3::<f64>(t380, t198 * t330 * t993 * t995 - t843 + t869 + t909 + t911 - t916, t826);
    let t1004 = piecewise3::<f64>(t120, t259 * t580 / F::new(2.0) + t826 * t30 / F::new(2.0), t381 * t581 / F::new(2.0) + t999 * t45 / F::new(2.0));
    (t999, t1004)
}
