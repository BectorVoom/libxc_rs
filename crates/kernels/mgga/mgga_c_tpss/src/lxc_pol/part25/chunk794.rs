//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 794/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk794<F: Float>(t30: F, t259: F, t379: F, t207: F, t5848: F, t1692: F, t1812: F, t198: F, t2439: F, t5853: F, t750: F, t821: F, t823: F, t1819: F, t45: F, t5539: F, t5591: F, t580: F, t581: F, t5849: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t5864 = t207 * t5848;
    let t5869 = -t1692 * t5853 * t821 + 3.0 * t1812 * t2439 * t750 + t198 * t5864 * t823;
    let t5870 = piecewise3(t380, 0.0, t5869);
    let t5875 = piecewise3(t120, 3.0 / 2.0 * t2439 * t1812 * t5539 + t1692 * t5849 * t30 / 2.0 - t1692 * t5853 * t5591 / 2.0 + t1692 * t1812 * t580 / 2.0, t1819 * t581 / 2.0 + t5870 * t45 / 2.0);
    (t5869, t5870, t5875)
}
