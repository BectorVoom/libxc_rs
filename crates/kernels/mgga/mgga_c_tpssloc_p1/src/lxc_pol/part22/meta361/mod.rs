//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta361 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1598;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1599;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1600;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1601;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1602;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta361<F: Float>(t10556: F, t10577: F, t13598: F, t13600: F, t13601: F, t13603: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F, t894: F, t901: F, t17157: F, t2826: F, t136: F, t5717: F, t699: F, t5720: F, t5723: F, t17187: F, t908: F, t13712: F, t13642: F, t13709: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17238: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t17271 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1598::<F>(t10556, t10577, t13598, t13600, t13601, t13603, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17272, t17274, t17279, t17280, t17286) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1599::<F>(t17271, t894, t901, t17157, t2826, t136, t5717, t699);
        let t17288 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1600::<F>(t5720, t699);
        let t17290 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1601::<F>(t5723, t699);
        let (t17292, t17293, t17295) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1602::<F>(t17187, t908, t136, t13598, t13712, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290);
        let t17297 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1603::<F>(t13642, t13709, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17238, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17295);
    (t17271, t17272, t17274, t17279, t17280, t17286, t17288, t17290, t17292, t17293, t17297)
}
