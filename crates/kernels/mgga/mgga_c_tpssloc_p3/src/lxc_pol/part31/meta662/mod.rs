//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1949;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta662<F: Float>(t28299: F, t81979: F, t28273: F, t6547: F, t28264: F, t17022: F, t1880: F, t214: F, t225: F, t258: F, t28272: F, t6562: F, t794: F, t25224: F, t25341: F, t6552: F, t23164: F, t7479: F, t86893: F, t16596: F, t86721: F, t1484: F, t584: F, t86753: F, t16949: F, t25014: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98993, t98995, t99003, t99019, t99022) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1949::<F>(t28299, t81979, t28273, t6547, t28264, t17022, t1880, t214, t225, t258, t28272, t6562, t794);
        let (t99033, t99036, t99049, t99053, t99056) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1950::<F>(t25224, t25341, t6552, t23164, t7479, t86893, t16596, t86721, t1484, t584, t86753, t16949, t25014);
    (t98993, t98995, t99003, t99019, t99022, t99033, t99036, t99049, t99053, t99056)
}
