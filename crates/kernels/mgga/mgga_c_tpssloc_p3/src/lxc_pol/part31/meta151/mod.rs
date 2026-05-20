//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk753;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk754;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk755;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk756;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta151<F: Float>(t4166: F, t816: F, t1500: F, t838: F, t842: F, t242: F, t2628: F, t812: F, t244: F, t67: F, t246: F, t120: F, t1509: F, t2632: F, t828: F, t1512: F, t2639: F, t249: F, t2571: F, t2602: F, t2603: F, t2618: F, t4152: F, t4155: F, t4159: F, t4163: F, t787: F, t831: F, t849: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4167, t4170, t4172, t4177, t4178, t4179, t4180) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk753::<F>(t4166, t816, t1500, t838, t842, t242, t2628, t812, t244, t67, t246);
        let t4181 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk754::<F>(t120, t1509);
        let t4182 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk755::<F>(t2632, t828);
        let t4184 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk756::<F>(t4180, t4181, t4182);
        let (t4187, t4189) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk757::<F>(t1512, t2639, t249, t2571, t2602, t2603, t2618, t4152, t4155, t4159, t4163, t4167, t4170, t4172, t4178, t4184, t787, t831, t849);
    (t4167, t4170, t4172, t4177, t4178, t4179, t4180, t4181, t4182, t4184, t4187, t4189)
}
