//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta606<F: Float>(t11687: F, t11697: F, t3577: F, t11877: F, t3576: F, t11647: F, t1203: F, t11859: F, t1222: F, t11797: F, t3490: F, t11172: F, t1227: F, t248: F, t3521: F, t11801: F, t204: F, t486: F, t1213: F, t1216: F, t11862: F, t13969: F, t11716: F, t44833: F, t44834: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44994, t44996, t45002, t45007, t45009, t45013) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189::<F>(t11687, t11697, t3577, t11877, t3576, t11647, t1203, t11859, t1222, t11797, t3490, t11172, t1227, t248, t3521);
        let (t45015, t45017, t45020, t45027, t45030) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190::<F>(t11801, t3490, t204, t486, t1213, t1216, t248, t11862, t1227, t13969, t11716, t44833, t44834);
    (t44994, t44996, t45002, t45007, t45009, t45013, t45015, t45017, t45020, t45027, t45030)
}
