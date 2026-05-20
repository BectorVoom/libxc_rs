//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2881;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta818<F: Float>(t2815: F, t60160: F, t136: F, t59659: F, t908: F, t17246: F, t699: F, t17249: F, t59763: F, t59767: F, t17252: F, t2403: F, t5717: F, t2826: F, t59676: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2881::<F>(t2815, t60160, t136, t59659, t908, t17246, t699, t17249, t59763, t59767, t17252, t2403, t5717);
        let (t60207, t60214) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882::<F>(t136, t2826, t59676, t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204);
    (t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207, t60214)
}
