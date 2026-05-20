//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta423<F: Float>(t1057: F, t69923: F, t1615: F, t883: F, t5866: F, t17906: F, t4644: F, t17607: F, t4571: F, t1011: F, t1019: F, t1040: F, t21482: F, t10876: F, t21396: F, t248: F, t3101: F, t1041: F, t21138: F, t3051: F, t21134: F, t14508: F, t17667: F, t17611: F, t4641: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69924, t70100, t70122, t70132, t70138, t70148, t70153) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250::<F>(t1057, t69923, t1615, t883, t5866, t17906, t4644, t17607, t4571, t1011, t1019, t1040, t21482);
        let (t70162, t70166, t70199, t70209, t70214) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1251::<F>(t10876, t21396, t248, t3101, t1041, t21138, t3051, t21134, t14508, t17667, t17611, t4641);
    (t69924, t70100, t70122, t70132, t70138, t70148, t70153, t70162, t70166, t70199, t70209, t70214)
}
