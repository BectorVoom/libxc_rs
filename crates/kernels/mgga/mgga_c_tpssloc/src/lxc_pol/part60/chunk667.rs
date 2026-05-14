//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 667/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk667<F: Float>(t3030: F, t3502: F, t478: F, t1209: F, t2141: F, t3540: F, t3: F, t7324: F, t2127: F, t3545: F, t461: F, t52: F, t1009: F, t3504: F, t3500: F, t7337: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24681 = t2141 * t3540 / 6912.0;
    let t24682 = t7324 * t3;
    let t24704 = t2127 * t3545 / 432.0;
    let t24719 = t52 * t461;
    let t24720 = t24719 * t1009;
    let t24721 = t7324 * t24720;
    let t24727 = t3502 * sigma2;
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    let t24732 = t7337 * t3504;
    (t24661, t24668, t24681, t24682, t24704, t24721, t24729, t24732)
}
