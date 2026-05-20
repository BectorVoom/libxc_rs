//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1047;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1048;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1049;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1050;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1051;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta307<F: Float>(t1113: F, t21749: F, t136: F, t11195: F, t11204: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F, t11147: F, t20234: F, t11145: F, t123: F, t11153: F, t3240: F, t21745: F, t3242: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21750, t21751, t21753) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046::<F>(t1113, t21749, t136, t11195, t11204, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747);
        let t21758 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1047::<F>(t11147, t20234);
        let (t21759, t21760) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1048::<F>(t11145, t21758, t123);
        let t21762 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1049::<F>(t11153, t20234);
        let (t21763, t21764) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1050::<F>(t21762, t3240, t123);
        let (t21766, t21767) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1051::<F>(t21745, t3240, t123);
        let t21769 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1052::<F>(t20234, t3242);
    (t21750, t21751, t21753, t21758, t21759, t21760, t21762, t21763, t21764, t21766, t21767, t21769)
}
