//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta621<F: Float>(t40817: F, t157: F, t41279: F, t4196: F, t4205: F, t9868: F, t13130: F, t2427: F, t41251: F, t10121: F, t13191: F, t1877: F, t2523: F, t39563: F, t39585: F, t39590: F, t39593: F, t4307: F, t4314: F, t193: F, t776: F, t12908: F, t13127: F, t3966: F, t4194: F, t607: F, t750: F, t12606: F, t184: F, t4202: F, t9912: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46331, t46334, t46336, t46338, t46339, t46340) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236::<F>(t40817, t157, t41279, t4196, t4205, t9868, t13130, t2427, t41251, t10121, t13191, t1877, t2523, t39563, t39585, t39590, t39593, t4307, t4314);
        let (t46341, t46345, t46349, t46353, t46355) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2237::<F>(t193, t776, t12908, t13127, t3966, t4194, t607, t750, t12606, t184, t4202, t9912);
    (t46331, t46334, t46336, t46338, t46339, t46340, t46341, t46345, t46349, t46353, t46355)
}
