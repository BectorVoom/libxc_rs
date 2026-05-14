//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1312/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1312<F: Float>(t30: F, t259: F, t379: F, t64808: F, t64855: F, t64762: F, t10353: F, t1289: F, t1742: F, t18231: F, t1992: F, t20003: F, t3431: F, t45: F, t5665: F, t581: F, t6201: F, t64310: F, t10667: F, t20011: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t64856 = t64808 + t64855;
    let t64857 = piecewise3(t380, t64762, t64856);
    let t64869 = piecewise3(t120, t64310, t64857 * t45 / 2.0 + t20003 * t581 + t6201 * t1992 / 2.0 + t18231 * t1289 / 2.0 + t5665 * t3431 + t1742 * t10353 / 2.0);
    let t64870 = t20011 * t10667;
    (t64856, t64869, t64870)
}
