//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 262/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk262<F: Float>(t1688: F, t300: F, t1147: F, t1156: F, t1694: F, t1164: F, t1420: F, t338: F, t1178: F, t1409: F, t1177: F, t1111: F, t1668: F, t457: F, t460: F, t974: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1701 = 0.19751673498613801407e-1 * t300 * t1688;
    let t1703 = t1147 * t1694 * t1156;
    let t1705 = 0.5848223622634646207e0 * t1164 * t1703;
    let t1706 = t1420 * t338;
    let t1709 = t1178 * t1409;
    let t1710 = t1177 * t1709;
    let t1714 = t1111 / 6.0 - t1668 / 6.0;
    let t1715 = t457 * t1714;
    let t1716 = t1715 * t460;
    let t1717 = t974 * t1716;
    (t1701, t1703, t1705, t1706, t1709, t1710, t1714, t1716, t1717)
}
