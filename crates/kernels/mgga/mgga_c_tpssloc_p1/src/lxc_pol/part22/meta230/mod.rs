//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1293;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1294;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta230<F: Float>(t831: F, t9671: F, t2617: F, t2638: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F, t2385: F, t686: F, t781: F, t685: F, t120: F, t118: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9672, t9674, t9688, t9689, t9691, t9692, t9694) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1293::<F>(t831, t9671, t2617, t2638, t116, t126, t136, t16, t2386, t625, t2385, t686, t781);
        let (t9695, t9697) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1294::<F>(t685, t9694, t120, t781);
        let t9698 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1295::<F>(t118, t9697);
    (t9672, t9674, t9688, t9689, t9691, t9692, t9694, t9695, t9697, t9698)
}
