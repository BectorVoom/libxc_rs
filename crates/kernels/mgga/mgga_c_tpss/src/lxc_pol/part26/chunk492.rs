//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 492/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk492<F: Float>(t1713: F, t30: F, t1692: F, t1712: F, t207: F, t198: F, t823: F, t33: F, t1165: F, t1688: F, t196: F, t488: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t1714 = t1713 * t30;
    let t1716 = t1692 * t1714 / 2.0;
    let t1739 = t207 * t1712;
    let t1741 = t198 * t1739 * t823;
    let t1746 = t1713 * t33;
    let t1748 = t1692 * t1746 / 2.0;
    let t1756 = 2.0 * t1165 * t1688;
    let t1759 = t488 * t196;
    let t1760 = t1759 * t197;
    (t1716, t1739, t1741, t1748, t1756, t1759, t1760)
}
