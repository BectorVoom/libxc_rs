//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta371<F: Float>(t3242: F, t415: F, t61: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t44722: F, t478: F, t11147: F, t3439: F, t11789: F, t820: F, t204: F, t486: F, t11716: F, t3503: F, t3584: F, t676: F, t221: F, t44483: F, t456: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44828, t44833, t44834, t44836, t44863, t44938) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1171::<F>(t3242, t415, t61, t42341, t44696, t42344, t483, t1210, t44722, t478, t11147, t3439);
        let (t44951, t45017, t45030, t45037, t45046, t45112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1172::<F>(t11789, t820, t204, t486, t11716, t44833, t44834, t3503, t3584, t676, t221, t44483, t456);
    (t44828, t44836, t44863, t44938, t44951, t45017, t45030, t45037, t45046, t45112)
}
