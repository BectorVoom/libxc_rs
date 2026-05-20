//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1115;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1116;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1117;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1118;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1119;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta247<F: Float>(t1883: F, t6547: F, t131: F, t209: F, t229: F, t1878: F, t214: F, t252: F, t225: F, t258: F, t776: F, t154: F, t16: F, t117: F, t206: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6549, t6551, t6552) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1115::<F>(t1883, t6547, t131, t209, t229, t1878);
        let t6553 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1116::<F>(t214, t252);
        let t6554 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1117::<F>(t225, t258);
        let t6555 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1118::<F>(t6554, t776);
        let (t6556, t6557, t6559) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1119::<F>(t6553, t6555, t6552, t154, t16);
        let (t6561, t6562) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1120::<F>(t117, t206, t67, t6559);
    (t6549, t6551, t6552, t6553, t6554, t6555, t6556, t6557, t6559, t6561, t6562)
}
