//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2202;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta704<F: Float>(t24996: F, t97890: F, t28860: F, t6876: F, t1307: F, t6324: F, t22574: F, t26162: F, t28835: F, t28830: F, t24995: F, t8643: F, t74060: F, t1388: F, t1983: F, t28238: F, t6999: F, t75214: F, t12461: F, t7752: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97892, t97893, t97897, t97899, t97905) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2202::<F>(t24996, t97890, t28860, t6876, t1307, t6324, t22574, t26162, t28835, t28830, t24995, t8643);
        let (t97910, t97914, t97916, t97919, t97920) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2203::<F>(t22574, t74060, t8643, t1388, t28830, t26162, t1983, t28238, t6999, t75214, t12461, t7752);
    (t97892, t97893, t97897, t97899, t97905, t97910, t97914, t97916, t97919, t97920)
}
