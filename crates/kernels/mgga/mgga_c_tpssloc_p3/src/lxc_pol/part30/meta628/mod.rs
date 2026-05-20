//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta628<F: Float>(t87328: F, t1516: F, t81769: F, t23133: F, t4261: F, t25111: F, t81782: F, t81783: F, t25115: F, t87229: F, t23132: F, t4166: F, t849: F, t81763: F, t23083: F, t25094: F, t23046: F, t4184: F, t812: F, t836: F, t242: F, t81816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87329, t87331, t87333, t87336, t87339, t87340) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2032::<F>(t87328, t1516, t81769, t23133, t4261, t25111, t81782, t81783, t25115, t87229, t23132, t4166);
        let (t87342, t87345, t87348, t87364, t87368) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2033::<F>(t849, t87340, t1516, t81763, t23083, t25094, t23046, t4184, t812, t836, t242, t81816);
    (t87329, t87331, t87333, t87336, t87339, t87340, t87342, t87345, t87348, t87364, t87368)
}
