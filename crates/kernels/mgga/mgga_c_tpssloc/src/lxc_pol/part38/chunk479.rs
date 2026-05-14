//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 479/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk479<F: Float>(t1099: F, t1671: F, t1122: F, t1655: F, t1131: F, t1134: F, t1662: F, t1665: F, t1668: F, t1137: F) -> (F, F, F, F) {
    let t1673 = 1.0 * t1099 * t1671;
    let t1675 = -t1122 + 0.17123333333333333333e-1 * t1655;
    let t1682 = 0.3529725e1 * t1662 - t1131 + 0.516475e0 * t1655 + 0.6311625e0 * t1665 - t1134 + 0.104195e0 * t1668;
    let t1683 = t1682 * t1137;
    (t1673, t1675, t1682, t1683)
}
