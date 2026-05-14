//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 502/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk502<F: Float>(t30: F, t259: F, t479: F, t1716: F, t1742: F, t45: F, t1713: F, t33: F, t1692: F, t1741: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t480 = t259 < t479;
    let t1745 = piecewise3(t120, t1716, t1742 * t45 / 2.0);
    let t1746 = t1713 * t33;
    let t1748 = t1692 * t1746 / 2.0;
    let t1749 = piecewise3(t480, 0.0, t1741);
    (t1745, t1748, t1749)
}
