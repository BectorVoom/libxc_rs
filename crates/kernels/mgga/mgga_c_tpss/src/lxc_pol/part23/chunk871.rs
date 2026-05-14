//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 871/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk871<F: Float>(t5: F, t30: F, t259: F, t379: F, t1675: F, t1861: F, t5966: F, t6073: F, t6077: F, t6080: F, t6472: F, t6475: F, t117: F, t1338: F, t1897: F, t6200: F, t1289: F, t1867: F, t45: F, t6160: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t6479 = piecewise3(t8, 0.0, -t6073 * t1861 / 6.0 + 5.0 / 6.0 * t5966 * t6077 + t6080 * t1861 / 3.0 - t1675 * t6472 / 6.0 - t1675 * t6475 / 6.0);
    let t6480 = t6479 * t117;
    let t6486 = t1897 * t1338;
    let t6489 = piecewise3(t380, 0.0, t6200);
    let t6494 = piecewise3(t120, t6160, t1867 * t1289 / 2.0 + t6489 * t45 / 2.0);
    (t6479, t6480, t6486, t6489, t6494)
}
