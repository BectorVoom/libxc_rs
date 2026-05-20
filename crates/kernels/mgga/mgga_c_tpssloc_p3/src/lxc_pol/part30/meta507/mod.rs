//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta507<F: Float>(t25650: F, t25651: F, t1615: F, t3128: F, t1022: F, t23678: F, t1015: F, t1011: F, t360: F, t1941: F, t4616: F, t23474: F, t23480: F, t23483: F, t23500: F, t23564: F, t25639: F, t25642: F, t25645: F, t378: F, t4585: F, t4609: F, t6717: F, t6747: F, t6765: F, t7583: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25652, t25653, t25654, t25655, t25658, t25660, t25661, t25664, t25672) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1826::<F>(t25650, t25651, t1615, t3128, t1022, t23678, t1015, t1011, t360, t1941, t4616, t23474, t23480, t23483, t23500, t23564, t25639, t25642, t25645, t378, t4585, t4609, t6717, t6747, t6765, t7583);
    (t25652, t25653, t25654, t25655, t25658, t25660, t25661, t25664, t25672)
}
