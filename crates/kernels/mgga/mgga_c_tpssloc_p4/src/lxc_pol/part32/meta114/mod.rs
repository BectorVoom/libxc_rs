//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk686;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk687;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk688;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk689;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta114<F: Float>(t2663: F, t756: F, t68: F, t845: F, t20: F, t61: F, t241: F, t244: F, t248: F, t238: F, t835: F, t841: F, t812: F, t849: F, t1891: F, t67: F, t225: F, t853: F, t257: F, t856: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2665, t2671, t2690) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk686::<F>(t2663, t756, t68, t845, t20, t61);
        let (t2691, t2693) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk687::<F>(t241, t2690, t244, t248);
        let (t2695, t2696, t2697) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk688::<F>(t238, t2693, t835, t841, t812);
        let (t2698, t2701, t2713) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk689::<F>(t2697, t849, t1891, t241, t67, t225, t853);
        let t2717 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk690::<F>(t257, t856);
    (t2665, t2671, t2690, t2691, t2693, t2695, t2696, t2697, t2698, t2701, t2713, t2717)
}
