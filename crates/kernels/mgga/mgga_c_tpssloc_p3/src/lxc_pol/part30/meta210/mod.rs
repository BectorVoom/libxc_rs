//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta210 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk987;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk988;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk989;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk990;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta210<F: Float>(t237: F, t5575: F, t1509: F, t2632: F, t819: F, t820: F, t1484: F, t232: F, t2645: F, t4181: F, t4212: F, t185: F, t5398: F, t707: F, t2373: F, t2377: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2665: F, t5497: F, t5498: F, t5501: F, t5506: F, t5521: F, t5524: F, t5525: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5576, t5584) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk987::<F>(t237, t5575, t1509);
        let t5585 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk988::<F>(t2632, t5584);
        let t5587 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk989::<F>(t5585, t819, t820);
        let (t5591, t5593) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk990::<F>(t1484, t232, t2645, t4181);
        let (t5596, t5597, t5599, t5600) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk991::<F>(t4212, t185, t5398, t707, t2373, t2377, t2408, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t5497, t5498, t5501, t5506, t5521, t5524, t5525);
    (t5576, t5584, t5585, t5587, t5591, t5593, t5596, t5597, t5599, t5600)
}
