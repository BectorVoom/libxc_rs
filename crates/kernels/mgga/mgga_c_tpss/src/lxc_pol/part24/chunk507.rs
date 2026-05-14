//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 507/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk507<F: Float>(t1760: F, t1779: F, t118: F, t1684: F, t1691: F, t1753: F, t1757: F, t485: F, t544: F, t3: F) -> (F, F, F) {
    let t1780 = t1760 * t1779;
    let t1781 = -t118 * t1753 - t1684 * t485 + t1757 * t544 - t1691 + t1780;
    let t1782 = t3 * t1781;
    let t1784 = param_d * t1781;
    (t1781, t1782, t1784)
}
