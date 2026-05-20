//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk937;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk938;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk939;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk940;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk941;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta147<F: Float>(t4433: F, t932: F, t1568: F, t2888: F, t931: F, t2766: F, t2892: F, t4335: F, t4340: F, t4345: F, t4349: F, t324: F, t1573: F, t942: F, t1581: F, t950: F, t2824: F, t2912: F, t2919: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F, t951: F, t1580: F, t2932: F, t1569: F, t2856: F, t2861: F, t2886: F, t2900: F, t2905: F, t2930: F, t311: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4408: F, t4411: F, t4416: F, t924: F, t933: F, t943: F, t952: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4434, t4437, t4438, t4446) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk937::<F>(t4433, t932, t1568, t2888, t931, t2766, t2892, t4335, t4340, t4345, t4349);
        let (t4447, t4449) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk938::<F>(t324, t4446, t1573, t942);
        let (t4454, t4471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk939::<F>(t1581, t950, t2766, t2824, t2912, t2919, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387, t4390, t4393);
        let t4472 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk940::<F>(t4471, t951);
        let t4475 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk941::<F>(t1580, t2932);
        let (t4476, t4479) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk942::<F>(t4475, t950, t1569, t1581, t2856, t2861, t2886, t2900, t2905, t2930, t311, t4353, t4356, t4358, t4361, t4398, t4402, t4408, t4411, t4416, t4434, t4438, t4447, t4449, t4454, t4472, t924, t933, t943, t952);
    (t4434, t4437, t4438, t4446, t4447, t4449, t4454, t4471, t4472, t4475, t4476, t4479)
}
