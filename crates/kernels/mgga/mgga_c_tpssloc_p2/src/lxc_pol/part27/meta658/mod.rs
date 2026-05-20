//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2299;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta658<F: Float>(t26411: F, t6914: F, t12420: F, t26331: F, t5335: F, t6976: F, t1351: F, t1992: F, t5318: F, t550: F, t16036: F, t22633: F, t3807: F, t12407: F, t22704: F, t22705: F, t5345: F, t54918: F, t22690: F, t552: F) -> (F, F, F, F, F, F, F, F) {
        let (t90760, t90763, t90770, t90774) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2299::<F>(t26411, t6914, t12420, t26331, t5335, t6976, t1351, t1992, t5318, t550, t16036, t22633, t3807);
        let (t90778, t90782, t90785, t90787) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2300::<F>(t12407, t22633, t5335, t6976, t22704, t22705, t5345, t1992, t54918, t550, t22690, t552);
    (t90760, t90763, t90770, t90774, t90778, t90782, t90785, t90787)
}
