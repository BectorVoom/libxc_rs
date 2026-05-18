//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 965/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk965<F: Float>(t1334: F, t2023: F, t3509: F, t600: F, t3533: F, t2083: F, t97: F, t105: F, t2091: F, t10281: F, t10282: F, t10283: F, t10284: F, t10285: F, t10286: F, t7656: F, t7659: F, t7665: F, t7668: F, t7676: F) -> (F, F, F, F, F, F) {
    let t13154 = t2023 * t1334;
    let t13157 = F::new(4.0) / F::new(3.0) * t600 * t3509;
    let t13159 = F::new(2.0) / F::new(3.0) * t600 * t3533;
    let t13181 = t97 * t2083;
    let t13202 = t105 * t2091;
    let t13296 = t10281 - t10282 - t7656 + t7659 + t10283 - t10284 - t7665 + t7668 + t10285 - t10286 - t7676;
    (t13154, t13157, t13159, t13181, t13202, t13296)
}
