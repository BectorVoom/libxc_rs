//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta675<F: Float>(t2018: F, t40611: F, t1845: F, t3698: F, t26161: F, t15868: F, t1983: F, t6996: F, t3734: F, t24995: F, t8643: F, t23831: F, t7458: F, t22480: F, t7461: F, t9348: F, t1774: F, t22479: F, t652: F, t7468: F, t15904: F, t22574: F, t33136: F, t12734: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91690, t91694, t91698, t91704) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264::<F>(t2018, t40611, t1845, t3698, t26161, t15868, t1983, t6996, t3734, t24995, t8643, t23831, t7458);
        let (t91706, t91708, t91713, t91715, t91718, t91722) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2265::<F>(t22480, t7458, t7461, t9348, t1774, t22479, t652, t7468, t15904, t22574, t33136, t12734);
    (t91690, t91694, t91698, t91704, t91706, t91708, t91713, t91715, t91718, t91722)
}
