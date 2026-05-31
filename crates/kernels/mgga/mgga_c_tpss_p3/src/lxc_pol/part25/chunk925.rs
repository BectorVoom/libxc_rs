//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 925/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk925<F: Float>(t1253: F, t3255: F, t7651: F, t7653: F, t7660: F, t7662: F, t7669: F, t7671: F, t3416: F, t577: F, t1286: F, t1980: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10193 = t3255 * t1253;
    let t10281 = F::cast_from(4.0_f64) * t7651;
    let t10282 = F::cast_from(12.0_f64) * t7653;
    let t10283 = F::cast_from(48.0_f64) * t7660;
    let t10284 = F::cast_from(80.0_f64) * t7662;
    let t10285 = F::cast_from(180.0_f64) * t7669;
    let t10286 = F::cast_from(252.0_f64) * t7671;
    let t10289 = t3416 * t577;
    let t10292 = t1286 * t1980;
    (t10193, t10281, t10282, t10283, t10284, t10285, t10286, t10289, t10292)
}
