//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2614;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta745<F: Float>(t11734: F, t15548: F, t1174: F, t14749: F, t3431: F, t1222: F, t15723: F, t11738: F, t13969: F, t15534: F, t3514: F, t53371: F, t1213: F, t15525: F, t248: F, t3570: F, t11813: F, t5018: F, t15749: F, t3577: F, t45124: F, t11835: F, t4889: F, t1725: F, t2402: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t53378, t53387, t53389, t53397, t53399) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2614::<F>(t11734, t15548, t1174, t14749, t3431, t1222, t15723, t11738, t13969, t15534, t3514, t53371);
        let (t53404, t53406, t53410, t53433, t53440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2615::<F>(t1213, t15525, t248, t3570, t11813, t5018, t15749, t3577, t45124, t11835, t4889, t1174, t1725, t2402);
    (t53378, t53387, t53389, t53397, t53399, t53404, t53406, t53410, t53433, t53440)
}
