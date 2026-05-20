//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2090;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta581<F: Float>(t11789: F, t820: F, t11737: F, t44857: F, t11647: F, t1203: F, t204: F, t486: F, t1213: F, t1216: F, t248: F, t11716: F, t44833: F, t44834: F, t3503: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F, t221: F, t44483: F, t456: F, t3575: F, t42386: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44951, t44965, t45002, t45017, t45020, t45030) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2090::<F>(t11789, t820, t11737, t44857, t11647, t1203, t204, t486, t1213, t1216, t248, t11716, t44833, t44834);
        let (t45037, t45044, t45046, t45112, t45113) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2091::<F>(t3503, t44833, t44834, t1174, t1197, t2402, t3584, t676, t221, t44483, t456, t3575, t42386);
    (t44951, t44965, t45002, t45017, t45020, t45030, t45037, t45044, t45046, t45112, t45113)
}
