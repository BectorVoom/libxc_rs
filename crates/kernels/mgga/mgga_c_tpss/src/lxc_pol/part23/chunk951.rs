//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 951/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk951<F: Float>(t1257: F, t73: F, t1219: F, t3357: F, t1270: F, t3387: F, t3202: F, t3205: F, t7651: F, t7653: F, t7660: F, t7662: F, t7669: F, t7671: F, t7656: F, t7659: F, t7665: F, t7668: F, t7676: F) -> (F, F, F, F, F, F, F) {
    let t10178 = t1257 * t1257;
    let t10179 = 1.0 / t10178;
    let t10180 = t73 * t10179;
    let t10204 = t1219 * t3357;
    let t10232 = t3387 * t1270;
    let t10236 = t3202 * t3205;
    let t10281 = 4.0 * t7651;
    let t10282 = 12.0 * t7653;
    let t10283 = 48.0 * t7660;
    let t10284 = 80.0 * t7662;
    let t10285 = 180.0 * t7669;
    let t10286 = 252.0 * t7671;
    let t10287 = t10281 + t10282 - t7656 - t7659 + t10283 + t10284 - t7665 - t7668 + t10285 + t10286 - t7676;
    (t10178, t10179, t10180, t10204, t10232, t10236, t10287)
}
