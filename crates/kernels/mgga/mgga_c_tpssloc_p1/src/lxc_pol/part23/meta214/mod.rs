//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk858;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta214<F: Float>(t2628: F, t836: F, t812: F, t242: F, t9972: F, t2638: F, t4166: F, t2629: F, t820: F, t9645: F, t2696: F, t1516: F, t9601: F, t68: F, t9971: F, t226: F, t1519: F, t2627: F, t1543: F, t2841: F, t1540: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13258, t13262, t13278, t13283, t13350, t13360, t13368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk858::<F>(t2628, t836, t812, t242, t9972, t2638, t4166, t2629, t820, t9645, t2696, t1516, t9601);
        let (t13397, t13416, t13520, t13598) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk859::<F>(t68, t9971, t226, t1519, t2627, t1543, t2841, t1540, t2394);
    (t13258, t13262, t13278, t13283, t13350, t13360, t13368, t13397, t13416, t13520, t13598)
}
