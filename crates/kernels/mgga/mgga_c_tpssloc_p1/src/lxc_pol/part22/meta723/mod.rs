//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta723<F: Float>(t21160: F, t699: F, t21167: F, t47705: F, t47707: F, t48103: F, t49139: F, t49144: F, t68442: F, t68444: F, t68446: F, t68448: F, t16558: F, t4342: F, t136: F, t908: F, t17156: F, t3966: F, t2826: F, t13527: F, t5398: F, t4337: F, t20234: F, t41666: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68452, t68454, t68457) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368::<F>(t21160, t699, t21167, t47705, t47707, t48103, t49139, t49144, t68442, t68444, t68446, t68448);
        let (t68458, t68460, t68462, t68464, t68466, t68468, t68470, t68472, t68477) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2369::<F>(t16558, t4342, t136, t908, t17156, t3966, t2826, t13527, t5398, t4337, t20234, t41666, t607);
    (t68452, t68454, t68457, t68458, t68460, t68462, t68464, t68466, t68468, t68470, t68472, t68477)
}
