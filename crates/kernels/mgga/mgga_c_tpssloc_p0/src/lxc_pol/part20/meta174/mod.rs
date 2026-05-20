//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1075;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1076;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1077;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1078;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1079;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1080;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1081;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta174<F: Float>(t120: F, t1509: F, t2632: F, t828: F, t4180: F, t1512: F, t2639: F, t249: F, t2571: F, t2602: F, t2603: F, t2618: F, t4152: F, t4155: F, t4159: F, t4163: F, t4167: F, t4170: F, t4172: F, t4178: F, t787: F, t831: F, t849: F, t2645: F, t2647: F, t157: F, t2658: F, t1409: F, t184: F, t607: F, t1474: F, t172: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t4181 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1075::<F>(t120, t1509);
        let t4182 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1076::<F>(t2632, t828);
        let t4184 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1077::<F>(t4180, t4181, t4182);
        let t4189 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1078::<F>(t1512, t2639, t249, t2571, t2602, t2603, t2618, t4152, t4155, t4159, t4163, t4167, t4170, t4172, t4178, t4184, t787, t831, t849);
        let t4191 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1079::<F>(t2645, t2647, t4181);
        let t4194 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1080::<F>(t157, t2658);
        let t4195 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1081::<F>(t1409, t184);
        let (t4196, t4198, t4199) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1082::<F>(t4195, t607, t4194, t1474, t172);
    (t4181, t4182, t4184, t4189, t4191, t4194, t4195, t4196, t4198, t4199)
}
