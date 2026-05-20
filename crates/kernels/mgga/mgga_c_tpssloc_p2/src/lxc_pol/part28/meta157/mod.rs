//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk797;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk798;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta157<F: Float>(t1176: F, t60: F, t1184: F, t1089: F, t460: F, t607: F, t3247: F, t461: F, t2244: F, t1177: F, t1178: F, t2250: F, t3293: F, t3295: F, t3299: F, t3302: F, t3305: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3448 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk797::<F>(t1176, t60);
        let (t3449, t3450, t3451) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk798::<F>(t1184, t3448, t1089, t460, t607);
        let (t3452, t3456, t3457, t3460, t3461, t3464, t3469) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk799::<F>(t3449, t3451, t3247, t461, t2244, t1177, t1178, t2250, t3293, t3295, t3299, t3302, t3305);
    (t3448, t3449, t3450, t3451, t3452, t3456, t3457, t3460, t3461, t3464, t3469)
}
