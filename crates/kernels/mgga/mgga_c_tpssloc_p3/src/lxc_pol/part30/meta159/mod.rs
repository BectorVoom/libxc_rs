//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk833;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk834;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta159<F: Float>(t4182: F, t4282: F, t1510: F, t2732: F, t4234: F, t860: F, t68: F, t814: F, t226: F, t829: F, t1519: F, t235: F, t4265: F, t1499: F, t1523: F, t1525: F, t255: F, t2617: F, t4162: F, t4166: F, t4281: F, t808: F, t812: F, t861: F, t863: F, t858: F, t1528: F, t259: F, t2597: F, t2713: F, t4143: F, t4145: F, t4147: F, t4149: F, t4266: F, t4268: F, t4273: F, t855: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4283, t4286, t4288, t4290, t4291) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk832::<F>(t4182, t4282, t1510, t2732, t4234, t860, t68, t814, t226);
        let (t4292, t4295, t4296, t4298, t4300) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk833::<F>(t4282, t829, t1519, t814, t235, t4265, t1499, t1523, t1525, t226, t255, t2617, t4162, t4166, t4281, t4283, t4286, t4288, t4291, t808, t812, t861, t863);
        let t4301 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk834::<F>(t4300, t858);
        let t4303 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk835::<F>(t1528, t259, t2597, t2713, t4143, t4145, t4147, t4149, t4266, t4268, t4273, t4301, t855, t866);
    (t4283, t4286, t4288, t4290, t4291, t4292, t4295, t4296, t4298, t4300, t4301, t4303)
}
