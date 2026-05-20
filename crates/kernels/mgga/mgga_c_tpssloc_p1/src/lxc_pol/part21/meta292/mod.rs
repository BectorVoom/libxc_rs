//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1606;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta292<F: Float>(t10294: F, t10544: F, t2840: F, t891: F, t275: F, t2843: F, t290: F, t2924: F, t2932: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10675, t10676, t10701, t10702) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1606::<F>(t10294, t10544, t2840, t891, t275);
        let (t10704, t10723, t10740, t10747, t10756) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1607::<F>(t2843, t290, t2924, t2932, t2860, t919, t2904, t938, t10629, t315);
    (t10675, t10676, t10701, t10702, t10704, t10723, t10740, t10747, t10756)
}
