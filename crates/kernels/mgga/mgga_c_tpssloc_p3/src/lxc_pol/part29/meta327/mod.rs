//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta327<F: Float>(t11539: F, t3442: F, t1174: F, t3247: F, t405: F, t974: F, t457: F, t63: F, t461: F, t221: F, t456: F, t1186: F, t698: F) -> (F, F, F, F, F, F, F) {
        let (t11541, t11545, t11546, t11552, t11553, t11556, t11557) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1384::<F>(t11539, t3442, t1174, t3247, t405, t974, t457, t63, t461, t221, t456, t1186, t698);
    (t11541, t11545, t11546, t11552, t11553, t11556, t11557)
}
