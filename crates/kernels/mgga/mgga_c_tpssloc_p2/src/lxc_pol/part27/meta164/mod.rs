//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk878;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta164<F: Float>(t1196: F, t2250: F, t974: F, t1176: F, t3247: F, t2244: F, t3242: F, t3439: F, t225: F, t3481: F, t68: F, t484: F, t121: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3551, t3552, t3556, t3557, t3561, t3562, t3565, t3566, t3567) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk878::<F>(t1196, t2250, t974, t1176, t3247, t2244, t3242, t3439, t225, t3481, t68, t484);
        let t3570 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk879::<F>(t121, t486);
    (t3551, t3552, t3556, t3557, t3561, t3562, t3565, t3566, t3567, t3570)
}
