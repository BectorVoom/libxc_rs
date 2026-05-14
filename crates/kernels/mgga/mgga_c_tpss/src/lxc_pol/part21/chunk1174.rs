//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1174/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1174<F: Float>(t30: F, t259: F, t379: F, t18192: F, t18196: F, t18200: F, t18230: F, t198: F, t2807: F, t2811: F, t330: F, t4023: F, t5652: F, t993: F, t995: F, t1742: F, t18066: F, t1992: F, t45: F, t5665: F, t581: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t18231 = piecewise3(t380, t18192 * t198 * t330 * t995 - 2.0 * t18196 * t4023 * t993 + 2.0 * t18200 * t2811 * t4023 - t2807 * t4023 * t5652, t18230);
    let t18238 = piecewise3(t120, t18066, t18231 * t45 / 2.0 + t5665 * t581 + t1742 * t1992 / 2.0);
    (t18231, t18238)
}
