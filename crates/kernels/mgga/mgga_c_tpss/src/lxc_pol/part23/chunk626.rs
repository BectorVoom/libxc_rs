//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 626/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk626<F: Float>(t30: F, t259: F, t379: F, t2445: F, t2817: F, t1991: F, t1992: F, t381: F, t45: F, t580: F, t581: F, t826: F, t999: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t2818 = piecewise3(t380, t2817, t2445);
    let t2825 = piecewise3(t120, t2445 * t30 / 2.0 + t826 * t580 + t259 * t1991 / 2.0, t2818 * t45 / 2.0 + t999 * t581 + t381 * t1992 / 2.0);
    let t2829 = -t1991;
    (t2818, t2825, t2829)
}
