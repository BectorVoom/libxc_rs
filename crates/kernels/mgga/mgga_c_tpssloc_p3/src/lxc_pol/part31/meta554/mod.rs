//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta554<F: Float>(t81281: F, t81072: F, t81074: F, t80825: F, t80847: F, t80885: F, t80899: F, t80956: F, t80970: F, t3787: F, t7191: F, t81146: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t84423, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84577, t84595) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1782::<F>(t81281, t81072, t81074, t80825, t80847, t80885, t80899, t80956, t80970, t3787, t7191, t81146);
    (t84423, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84577, t84595)
}
