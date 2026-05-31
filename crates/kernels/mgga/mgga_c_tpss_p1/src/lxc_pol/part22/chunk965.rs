//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 965/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk965<F: Float>(t7660: F, t7662: F, t7669: F, t7671: F, t10281: F, t10282: F, t7656: F, t7659: F, t7665: F, t7668: F, t7676: F, t3416: F, t577: F) -> (F, F) {
    let t10283 = F::cast_from(48.0_f64) * t7660;
    let t10284 = F::cast_from(80.0_f64) * t7662;
    let t10285 = F::cast_from(180.0_f64) * t7669;
    let t10286 = F::cast_from(252.0_f64) * t7671;
    let t10287 = t10281 + t10282 - t7656 - t7659 + t10283 + t10284 - t7665 - t7668 + t10285 + t10286 - t7676;
    let t10289 = t3416 * t577;
    (t10287, t10289)
}
