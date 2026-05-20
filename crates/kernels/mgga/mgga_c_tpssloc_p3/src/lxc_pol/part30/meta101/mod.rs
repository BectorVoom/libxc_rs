//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk648;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk649;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk650;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta101<F: Float>(t94: F, t102: F, t177: F, t738: F, t745: F, t746: F, t761: F, t118: F, t187: F, t677: F, t763: F) -> (F, F, F, F, F, F, F, F) {
        let (t2341, t2349, t2368, t2369) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk648::<F>(t94, t102, t177, t738, t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk649::<F>(t2368, t2369, t746);
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk650::<F>(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk651::<F>(t677, t763);
    (t2341, t2349, t2368, t2369, t2371, t2373, t2374, t2375)
}
