//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 936/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk936<F: Float>(t242: F, t527: F, t8200: F, t525: F, t1257: F, t73: F, t1253: F, t3255: F, t7651: F, t7653: F, t7660: F, t7662: F, t7669: F, t7671: F, t3416: F, t577: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10164 = t8200 * t527 * t242;
    let t10166 = 595.0 / 10368.0 * t525 * t10164;
    let t10178 = t1257 * t1257;
    let t10179 = 1.0 / t10178;
    let t10180 = t73 * t10179;
    let t10193 = t3255 * t1253;
    let t10281 = 4.0 * t7651;
    let t10282 = 12.0 * t7653;
    let t10283 = 48.0 * t7660;
    let t10284 = 80.0 * t7662;
    let t10285 = 180.0 * t7669;
    let t10286 = 252.0 * t7671;
    let t10289 = t3416 * t577;
    (t10164, t10166, t10178, t10179, t10180, t10193, t10281, t10282, t10283, t10284, t10285, t10286, t10289)
}
