//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta625<F: Float>(t461: F, t607: F, t1009: F, t7324: F, t24722: F, t24658: F, t27635: F, t24663: F, t3503: F, t1210: F, t24669: F, t1222: F, t24677: F) -> (F, F, F, F, F, F, F) {
        let (t86259, t86261, t86262, t86264, t86266, t86269, t86273) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2067::<F>(t461, t607, t1009, t7324, t24722, t24658, t27635, t24663, t3503, t1210, t24669, t1222, t24677);
    (t86259, t86261, t86262, t86264, t86266, t86269, t86273)
}
