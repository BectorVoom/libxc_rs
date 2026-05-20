//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1171;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1172;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta200<F: Float>(t3151: F, t5392: F, t974: F, t5398: F, t998: F, t3146: F, t1044: F, t248: F, t5681: F, t225: F, t5848: F, t68: F, t369: F, t1539: F, t1616: F, t3071: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5884, t5885, t5889, t5890, t5893, t5894, t5900) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1171::<F>(t3151, t5392, t974, t5398, t998, t3146, t1044, t248, t5681);
        let t5903 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1172::<F>(t225, t5848);
        let (t5904, t5905, t5908, t5909) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1173::<F>(t5903, t68, t369, t1539, t1616, t3071);
    (t5884, t5885, t5889, t5890, t5893, t5894, t5900, t5903, t5904, t5905, t5908, t5909)
}
