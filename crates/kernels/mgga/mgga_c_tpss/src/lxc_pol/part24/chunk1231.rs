//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1231/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1231<F: Float>(t30: F, t259: F, t379: F, t1485: F, t18200: F, t198: F, t19960: F, t21440: F, t21476: F, t330: F, t4023: F, t5039: F, t5043: F, t5652: F, t995: F, t1289: F, t1742: F, t21366: F, t45: F, t4579: F, t6201: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t21477 = piecewise3(t380, t198 * t21440 * t330 * t995 - 2.0 * t1485 * t19960 * t4023 + 2.0 * t18200 * t4023 * t5043 - t4023 * t5039 * t5652, t21476);
    let t21484 = piecewise3(t120, t21366, t21477 * t45 / 2.0 + t6201 * t1289 + t1742 * t4579 / 2.0);
    (t21477, t21484)
}
