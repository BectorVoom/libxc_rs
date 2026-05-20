//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2022/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2022<F: Float>(t7291: F, t85660: F, t11605: F, t225: F, t7303: F, t1235: F, t24594: F, t1176: F, t1184: F, t24847: F, t974: F, t1009: F, t460: F) -> (F, F, F, F, F, F) {
    let t85661 = t85660 * t7291;
    let t85674 = t225 * t11605;
    let t85701 = t85660 * t7303;
    let t85807 = t24594 * t1235;
    let t85820 = t24847 * t974 * t1176 * t1184;
    let t85821 = t460 * t1009;
    (t85661, t85674, t85701, t85807, t85820, t85821)
}
