//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1185;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta195<F: Float>(t1088: F, t4733: F, t123: F, t3237: F, t3238: F, t4721: F, t4726: F, t4731: F, t423: F, t1098: F, t1657: F, t1119: F, t1671: F, t3259: F, t1117: F, t3264: F, t1661: F, t3270: F, t1102: F, t3274: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4734, t4735, t4737, t4739, t4740, t4742) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1185::<F>(t1088, t4733, t123, t3237, t3238, t4721, t4726, t4731, t423, t1098, t1657, t1119);
        let (t4744, t4745, t4747, t4748, t4749, t4756) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1186::<F>(t1671, t3259, t1117, t3264, t1661, t3270, t1102, t3238, t3274, t4721, t4726, t4731, t4735);
    (t4734, t4735, t4737, t4739, t4740, t4742, t4744, t4745, t4747, t4748, t4749, t4756)
}
