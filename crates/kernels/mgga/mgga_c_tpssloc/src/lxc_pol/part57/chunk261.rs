//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 261/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk261<F: Float>(t1137: F, t1682: F, t1141: F, t1655: F, t449: F, t1150: F, t1153: F, t1662: F, t1665: F, t1668: F, t1156: F, t1129: F, t1148: F, t1659: F, t1673: F, t1675: F, t300: F, t436: F) -> (F, F, F, F, F, F) {
    let t1683 = t1682 * t1137;
    let t1687 = -t1141 + 0.92708333333333333333e-2 * t1655;
    let t1688 = t1687 * t449;
    let t1694 = 0.258925e1 * t1662 - t1150 + 0.301925e0 * t1655 + 0.16504875e0 * t1665 - t1153 + 0.82785e-1 * t1668;
    let t1695 = t1694 * t1156;
    let t1699 = t300 * (-0.310907e-1 * t1675 * t436 + 1.0 * t1129 * t1683 + t1659 - t1673 - 0.19751673498613801407e-1 * t1688 + 0.5848223622634646207e0 * t1148 * t1695);
    (t1683, t1687, t1688, t1694, t1695, t1699)
}
