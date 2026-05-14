//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 463/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk463<F: Float>(t30: F, t33: F, t1614: F, t489: F, t1613: F, t187: F, t1197: F, t1288: F, t1201: F, t1497: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1615 = t489 * t1614;
    let t1617 = 0.19751673498613801407e-1 * t1613 * t187;
    let t1620 = piecewise3(t31, 0.0, 2.0 / 3.0 * t1197 * t1288);
    let t1623 = piecewise3(t34, 0.0, 2.0 / 3.0 * t1201 * t1497);
    let t1625 = t1620 / 2.0 + t1623 / 2.0;
    (t1615, t1617, t1625)
}
