//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 742/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk742<F: Float>(t33: F, t259: F, t479: F, t4818: F, t5305: F, t1289: F, t1402: F, t1497: F, t1594: F, t4579: F, t481: F, t5059: F, t57: F, t5055: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t5306 = piecewise3(t480, t5305, t4818);
    let t5313 = piecewise3(t386, t4818 * t33 / 2.0 + t1402 * t1497 + t259 * t5059 / 2.0, t5306 * t57 / 2.0 - t1594 * t1289 - t481 * t4579 / 2.0);
    let t5314 = t5055 + t5313;
    (t5306, t5314)
}
