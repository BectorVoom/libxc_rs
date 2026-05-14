//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 963/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk963<F: Float>(t7669: F, t7671: F, t10281: F, t10282: F, t10283: F, t10284: F, t7656: F, t7659: F, t7665: F, t7668: F, t7676: F, t3416: F, t577: F, t1286: F, t1980: F, t1317: F, t1982: F) -> (F, F, F, F) {
    let t10285 = 180.0 * t7669;
    let t10286 = 252.0 * t7671;
    let t10287 = t10281 + t10282 - t7656 - t7659 + t10283 + t10284 - t7665 - t7668 + t10285 + t10286 - t7676;
    let t10289 = t3416 * t577;
    let t10292 = t1286 * t1980;
    let t10303 = t1317 * t1982;
    (t10287, t10289, t10292, t10303)
}
