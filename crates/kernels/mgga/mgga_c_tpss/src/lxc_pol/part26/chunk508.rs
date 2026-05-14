//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 508/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk508<F: Float>(t30: F, t1716: F, t1867: F, t45: F, t331: F, t55: F, t136: F, t452: F, dens_threshold: F, rho0: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t1870 = piecewise3(t120, t1716, t1867 * t45 / 2.0);
    let t1871 = t55 * t331;
    let t1872 = t1871 * t136;
    let t1875 = t452 * sigma2;
    (t1870, t1871, t1872, t1875)
}
