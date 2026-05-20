//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1395;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta322<F: Float>(t11539: F, t3442: F, t1174: F, t3247: F, t405: F, t974: F, t457: F, t63: F, t461: F, t221: F, t456: F, t1186: F, t698: F, t135: F, t3471: F, t1184: F, t4899: F, t3242: F, t460: F, t2244: F, t3448: F, t3469: F, t2250: F, t3450: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11541, t11545, t11546, t11552, t11556, t11557) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1395::<F>(t11539, t3442, t1174, t3247, t405, t974, t457, t63, t461, t221, t456, t1186, t698);
        let (t11558, t11561, t11569, t11570, t11571, t11575, t11579) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1396::<F>(t11557, t1174, t135, t3471, t1184, t4899, t3242, t460, t2244, t3448, t3469, t2250, t3450);
    (t11541, t11545, t11546, t11552, t11556, t11558, t11561, t11569, t11570, t11571, t11575, t11579)
}
