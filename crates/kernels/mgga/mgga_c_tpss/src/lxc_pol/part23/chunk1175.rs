//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1175/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1175<F: Float>(t33: F, t259: F, t479: F, t3154: F, t6040: F, t1889: F, t9519: F, t1151: F, t1153: F, t18230: F, t19164: F, t198: F, t3147: F, t3151: F, t330: F, t4023: F, t6044: F, t18278: F, t1893: F, t1992: F, t57: F, t581: F, t6048: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t19168 = t6040 * t3154;
    let t19172 = t1889 * t9519;
    let t19179 = piecewise3(t480, t1153 * t19164 * t198 * t330 - 2.0 * t1151 * t19168 * t4023 + 2.0 * t19172 * t3151 * t4023 - t3147 * t4023 * t6044, t18230);
    let t19186 = piecewise3(t386, t18278, t19179 * t57 / 2.0 - t6048 * t581 - t1893 * t1992 / 2.0);
    (t19168, t19172, t19179, t19186)
}
