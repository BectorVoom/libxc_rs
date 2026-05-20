//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk673;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk674;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk675;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk676;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta125<F: Float>(t1089: F, t460: F, t607: F, t3449: F, t3247: F, t461: F, t2244: F, t1177: F, t1178: F, t2250: F, t3293: F, t3295: F, t3299: F, t3302: F, t3305: F, t457: F, t974: F, t1184: F, t1174: F, t3430: F, t3433: F, t3436: F, t3443: F, t3447: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3450, t3451) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk673::<F>(t1089, t460, t607);
        let (t3452, t3455, t3456, t3457, t3460, t3461, t3469) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk674::<F>(t3449, t3451, t3247, t461, t2244, t1177, t1178, t2250, t3293, t3295, t3299, t3302, t3305);
        let (t3471, t3472, t3475) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk675::<F>(t3469, t457, t460, t974, t1184);
        let (t3477, t3481) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk676::<F>(t3475, t457, t460, t974, t1174, t3430, t3433, t3436, t3443, t3447, t3452, t3457, t3461, t3472);
    (t3450, t3451, t3455, t3456, t3460, t3469, t3471, t3475, t3477, t3481)
}
