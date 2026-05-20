//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta687<F: Float>(t11702: F, t5002: F, t11708: F, t15502: F, t15506: F, t13969: F, t15554: F, t3506: F, t10469: F, t1720: F, t10471: F, t11737: F, t11651: F, t15507: F, t11709: F, t1174: F, t11741: F, t1177: F, t11805: F, t11809: F, t15622: F, t15627: F, t15631: F, t1737: F, t44858: F, t44896: F, t45080: F, t4582: F, t4978: F, t5005: F, t50865: F, t50869: F, t52659: F, t15621: F, t11791: F, t11697: F, t15477: F, t3577: F, t11677: F, t15027: F, t11680: F, t11684: F, t11751: F, t1227: F, t15740: F, t3440: F, t45997: F, t4889: F, t4972: F, t50873: F, t50884: F, t50959: F, t50964: F) -> (F, F, F, F, F, F, F, F) {
        let (t52801, t52810, t52813, t52817, t52834, t52835, t52836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2602::<F>(t11702, t5002, t11708, t15502, t15506, t13969, t15554, t3506, t10469, t1720, t10471, t11737);
        let t52853 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603::<F>(t11651, t15507, t11709, t1174, t11741, t1177, t11805, t11809, t15622, t15627, t15631, t1737, t3506, t44858, t44896, t45080, t4582, t4978, t5005, t50865, t50869, t52659, t52836);
        let t52886 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604::<F>(t13969, t15621, t3506, t11791, t5005, t11697, t15477, t3577, t11677, t15027, t11680, t11684, t1174, t11751, t1177, t1227, t15740, t3440, t4582, t45997, t4889, t4972, t50873, t50884, t50959, t50964);
    (t52801, t52810, t52813, t52817, t52834, t52835, t52853, t52886)
}
