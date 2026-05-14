//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1238/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1238<F: Float>(t30: F, t259: F, t379: F, t1786: F, t5470: F, t1668: F, t6290: F, t6293: F, t1688: F, t4637: F, t547: F, t1338: F, t20124: F, t4674: F, t5772: F, t117: F, t21190: F, t21476: F, t1289: F, t1867: F, t21366: F, t45: F, t4579: F, t6489: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t21555 = 3.0 * t5470 * t1786;
    let t21557 = 12.0 * t1668 * t6290;
    let t21559 = 6.0 * t1668 * t6293;
    let t21560 = t4637 * t1688;
    let t21562 = 6.0 * t547 * t21560;
    let t21563 = t20124 * t1338;
    let t21565 = 12.0 * t547 * t21563;
    let t21566 = t5772 * t4674;
    let t21568 = 6.0 * t547 * t21566;
    let t21569 = t117 * t21190;
    let t21571 = 3.0 * t547 * t21569;
    let t21988 = piecewise3(t380, 0.0, t21476);
    let t21995 = piecewise3(t120, t21366, t21988 * t45 / 2.0 + t6489 * t1289 + t1867 * t4579 / 2.0);
    (t21555, t21557, t21559, t21560, t21562, t21563, t21565, t21566, t21568, t21569, t21571, t21988, t21995)
}
