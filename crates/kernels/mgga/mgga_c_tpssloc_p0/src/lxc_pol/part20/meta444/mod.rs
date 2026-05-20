//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1888;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta444<F: Float>(t11285: F, t3377: F, t14853: F, t1164: F, t300: F, t4832: F, t1166: F, t3419: F, t4869: F, t11180: F, t1671: F, t3259: F, t4782: F, t14704: F, t14710: F, t14722: F, t11215: F, t11217: F, t14720: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14854, t14855, t14857, t14858, t14860, t14862, t14864, t14866) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1888::<F>(t11285, t3377, t14853, t1164, t300, t4832, t1166, t3419, t4869, t11180, t1671, t3259, t4782);
        let (t14868, t14870, t14887) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1889::<F>(t14704, t14710, t14722, t11215, t11217, t14720, t14733, t14738, t14742, t14746, t14751, t14755, t14766);
    (t14854, t14855, t14857, t14858, t14860, t14862, t14864, t14866, t14868, t14870, t14887)
}
