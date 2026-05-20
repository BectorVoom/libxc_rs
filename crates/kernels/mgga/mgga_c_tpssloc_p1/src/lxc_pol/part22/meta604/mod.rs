//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2126;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta604<F: Float>(t10828: F, t300: F, t2930: F, t3030: F, t4552: F, t3032: F, t3129: F, t42875: F, t4338: F, t973: F, t13965: F, t3114: F, t14202: F, t3117: F, t10890: F, t14507: F, t3038: F, t1020: F, t10508: F, t248: F, t4650: F, t3109: F, t247: F, t677: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49532, t49541, t49649, t49650, t49651, t49662, t49690) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2126::<F>(t10828, t300, t2930, t3030, t4552, t3032, t3129, t42875, t4338, t973, t13965, t3114);
        let (t49691, t49693, t49743, t49771, t49819, t49831) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127::<F>(t49690, t14202, t3117, t10890, t14507, t3038, t49650, t1020, t10508, t248, t4650, t13965, t3109);
        let (t49832, t49850) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2128::<F>(t49831, t247, t677);
    (t49532, t49541, t49649, t49651, t49662, t49691, t49693, t49743, t49771, t49819, t49832, t49850)
}
