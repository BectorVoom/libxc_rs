//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 339/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk339<F: Float>(t1100: F, t1661: F, t1107: F, t1113: F, t1653: F, t136: F, t1105: F, t1112: F, t1655: F, t1118: F, t1099: F, t1122: F, t1131: F, t1134: F, t1137: F, t1141: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1662 = t1100 * t1661;
    let t1665 = t1107 * t1661;
    let t1667 = t1113 * t1653;
    let t1668 = t136 * t1667;
    let t1670 = 0.1898925e1 * t1662 - t1105 + 0.29896666666666666667e0 * t1655 + 0.3071625e0 * t1665 - t1112 + 0.82156666666666666667e-1 * t1668;
    let t1671 = t1670 * t1118;
    let t1673 = 1.0 * t1099 * t1671;
    let t1675 = -t1122 + 0.17123333333333333333e-1 * t1655;
    let t1682 = 0.3529725e1 * t1662 - t1131 + 0.516475e0 * t1655 + 0.6311625e0 * t1665 - t1134 + 0.104195e0 * t1668;
    let t1683 = t1682 * t1137;
    let t1687 = -t1141 + 0.92708333333333333333e-2 * t1655;
    (t1662, t1665, t1667, t1668, t1670, t1671, t1673, t1675, t1682, t1683, t1687)
}
