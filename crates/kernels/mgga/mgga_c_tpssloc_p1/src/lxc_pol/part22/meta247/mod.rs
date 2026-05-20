//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1356;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1357;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1358;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta247<F: Float>(t10544: F, t2841: F, t888: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10636, t10655, t10660, t10661) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1356::<F>(t10544, t2841, t888, t2840, t287, t275);
        let (t10675, t10676, t10701, t10702) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1357::<F>(t10294, t10544, t2840, t891, t275);
        let (t10704, t10740, t10747, t10756) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1358::<F>(t2843, t290, t2860, t919, t2904, t938, t10629, t315);
    (t10636, t10655, t10660, t10661, t10675, t10676, t10701, t10702, t10704, t10740, t10747, t10756)
}
