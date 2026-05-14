//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 341/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk341<F: Float>(t1156: F, t1694: F, t1129: F, t1148: F, t1659: F, t1673: F, t1675: F, t1683: F, t1688: F, t300: F, t436: F, t1147: F, t1164: F, t1420: F, t338: F) -> (F, F, F, F, F, F) {
    let t1695 = t1694 * t1156;
    let t1699 = t300 * (-0.310907e-1 * t1675 * t436 + 1.0 * t1129 * t1683 + t1659 - t1673 - 0.19751673498613801407e-1 * t1688 + 0.5848223622634646207e0 * t1148 * t1695);
    let t1701 = 0.19751673498613801407e-1 * t300 * t1688;
    let t1703 = t1147 * t1694 * t1156;
    let t1705 = 0.5848223622634646207e0 * t1164 * t1703;
    let t1706 = t1420 * t338;
    (t1695, t1699, t1701, t1703, t1705, t1706)
}
