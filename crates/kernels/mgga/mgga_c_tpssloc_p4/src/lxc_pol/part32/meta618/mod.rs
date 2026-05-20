//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2021;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta618<F: Float>(t1240: F, t3242: F, t3247: F, t2127: F, t82631: F, t7291: F, t11605: F, t225: F, t7303: F, t1235: F, t24594: F, t1176: F, t1184: F, t24847: F, t974: F, t1009: F, t460: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t85642, t85652, t85660) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2021::<F>(t1240, t3242, t3247, t2127, t82631);
        let (t85661, t85674, t85701, t85807, t85820, t85821) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2022::<F>(t7291, t85660, t11605, t225, t7303, t1235, t24594, t1176, t1184, t24847, t974, t1009, t460);
    (t85642, t85652, t85660, t85661, t85674, t85701, t85807, t85820, t85821)
}
