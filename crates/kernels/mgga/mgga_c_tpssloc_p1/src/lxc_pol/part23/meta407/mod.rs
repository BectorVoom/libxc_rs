//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta407<F: Float>(t3447: F, t4904: F, t51968: F, t3428: F, t6109: F, t1174: F, t6146: F, t698: F, t6140: F, t11529: F, t6130: F, t15299: F, t4889: F) -> (F, F, F, F, F, F) {
        let (t64821, t64878, t64881, t64885, t64979, t65002) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1220::<F>(t3447, t4904, t51968, t3428, t6109, t1174, t6146, t698, t6140, t11529, t6130, t15299, t4889);
    (t64821, t64878, t64881, t64885, t64979, t65002)
}
