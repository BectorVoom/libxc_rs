//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1385;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta328<F: Float>(t11557: F, t1174: F, t135: F, t3471: F, t1184: F, t4899: F, t3242: F, t460: F, t2244: F, t3448: F, t3469: F, t2250: F, t3450: F, t3247: F, t1176: F, t134: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11558, t11561, t11569, t11570, t11571, t11575, t11579) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1385::<F>(t11557, t1174, t135, t3471, t1184, t4899, t3242, t460, t2244, t3448, t3469, t2250, t3450);
        let (t11583, t11584, t11588) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1386::<F>(t3247, t460, t2244, t1176, t134);
    (t11558, t11561, t11569, t11570, t11571, t11575, t11579, t11583, t11584, t11588)
}
