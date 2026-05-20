//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1922;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta495<F: Float>(t10214: F, t21468: F, t20234: F, t2980: F, t977: F, t21126: F, t4518: F, t13909: F, t17784: F, t17809: F, t21430: F, t21433: F, t21447: F, t21453: F, t21459: F, t21463: F, t2986: F, t973: F, t21429: F, t225: F, t68: F, t369: F, t14211: F, t17712: F, t4582: F, t21122: F, t2979: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21469, t21472, t21473, t21476, t21479) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1922::<F>(t10214, t21468, t20234, t2980, t977, t21126, t4518, t13909, t17784, t17809, t21430, t21433, t21447, t21453, t21459, t21463, t2986, t973);
        let (t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1923::<F>(t21429, t21479, t225, t68, t369, t14211, t17712, t4582, t21126, t977, t21122, t2979);
    (t21469, t21472, t21473, t21476, t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493)
}
