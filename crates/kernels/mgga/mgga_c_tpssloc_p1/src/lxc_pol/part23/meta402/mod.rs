//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta402<F: Float>(t10224: F, t5828: F, t973: F, t42875: F, t5817: F, t10508: F, t248: F, t3130: F, t5873: F, t3030: F, t5848: F, t3032: F, t3129: F, t3038: F, t1041: F, t10868: F, t5685: F, t18086: F, t3069: F, t10482: F, t5872: F, t5681: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t61597, t61600, t61663, t61734, t61735) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211::<F>(t10224, t5828, t973, t42875, t5817, t10508, t248, t3130, t5873, t3030, t5848, t3032);
        let (t61736, t61739, t61782, t61950, t62079, t62137) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212::<F>(t3129, t61735, t3038, t1041, t10868, t248, t5685, t18086, t3069, t10482, t5872, t5681);
    (t61597, t61600, t61663, t61734, t61736, t61739, t61782, t61950, t62079, t62137)
}
