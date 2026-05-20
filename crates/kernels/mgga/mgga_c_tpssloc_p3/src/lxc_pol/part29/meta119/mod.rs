//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk710;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk711;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk712;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk713;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk714;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk715;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta119<F: Float>(t225: F, t2666: F, t68: F, t845: F, t2379: F, t2553: F, t824: F, t228: F, t230: F, t822: F, t825: F, t232: F, t819: F, t820: F, t2631: F, t20: F, t61: F, t241: F, t244: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2667, t2672, t2675, t2678) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk710::<F>(t225, t2666, t68, t845, t2379, t2553, t824, t228, t230, t822, t825);
        let t2679 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk711::<F>(t232, t2678);
        let t2681 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk712::<F>(t2679, t819, t820);
        let t2684 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk713::<F>(t232, t2631);
        let t2686 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk714::<F>(t2684, t819, t820);
        let t2690 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk715::<F>(t20, t61);
        let (t2691, t2693) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk716::<F>(t241, t2690, t244, t248);
    (t2667, t2672, t2675, t2678, t2679, t2681, t2684, t2686, t2690, t2691, t2693)
}
