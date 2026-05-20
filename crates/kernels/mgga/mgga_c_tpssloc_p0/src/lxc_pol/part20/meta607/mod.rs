//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta607<F: Float>(t3503: F, t44833: F, t44834: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F, t1227: F, t248: F, t3243: F, t1011: F, t1212: F, t44706: F, t11692: F, t11693: F, t11697: F, t11853: F, t1213: F, t3570: F, t11163: F, t3521: F, t221: F, t44483: F, t456: F) -> (F, F, F, F, F, F, F, F) {
        let (t45037, t45044, t45049, t45080) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2191::<F>(t3503, t44833, t44834, t1174, t1197, t2402, t3584, t676, t1227, t248, t3243, t1011, t1212, t44706);
        let (t45086, t45102, t45108, t45112) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2192::<F>(t11692, t11693, t11697, t11853, t1213, t248, t3570, t11163, t1227, t3521, t221, t44483, t456);
    (t45037, t45044, t45049, t45080, t45086, t45102, t45108, t45112)
}
