//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 951/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk951<F: Float>(t21510: F, t4588: F, t4582: F, t10970: F, t21130: F, t248: F, t1616: F, t5681: F, t3071: F, t1539: F, t5873: F, t10403: F, t1041: F, t13966: F, t13995: F, t17621: F, t17625: F, t17656: F, t17660: F, t17662: F, t17668: F, t21503: F, t3039: F, t3070: F, t5909: F) -> (F,) {
    let t21511 = t4588 * t21510;
    let t21512 = t4582 * t21511;
    let t21516 = t248 * t10970 * t21130;
    let t21519 = t5681 * t1616;
    let t21520 = t3071 * t21519;
    let t21525 = t5873 * t1539;
    let t21526 = t3071 * t21525;
    let t21529 = t17621 / 216.0 - t13966 / 4608.0 - t17625 / 144.0 - t3039 * t21503 / 1024.0 - t17656 / 1536.0 + t17660 / 2304.0 + t17662 / 768.0 + t17668 / 768.0 + 5.0 / 4608.0 * t1041 * t21512 + 5.0 / 5184.0 * t1041 * t21516 - t3070 * t21520 / 768.0 + t13995 * t5909 / 768.0 + t10403 * t21526 / 768.0;
    (t21529,)
}
