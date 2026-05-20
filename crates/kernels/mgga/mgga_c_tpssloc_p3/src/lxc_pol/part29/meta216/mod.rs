//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1045;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1046;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta216<F: Float>(t1137: F, t4819: F, t1682: F, t3359: F, t1136: F, t3238: F, t3363: F, t4721: F, t4726: F, t4731: F, t4735: F, t449: F, t1147: F, t1687: F, t1155: F, t1695: F, t3295: F, t3383: F, t3390: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F, t1156: F, t1694: F, t3403: F, t1129: F, t1138: F, t1148: F, t1157: F, t1683: F, t3327: F, t3332: F, t3357: F, t3371: F, t3376: F, t3401: F, t436: F, t4739: F, t4742: F, t4744: F, t4747: F, t4784: F, t4788: F, t4794: F, t4797: F, t4802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4820, t4823, t4824, t4832, t4833) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1045::<F>(t1137, t4819, t1682, t3359, t1136, t3238, t3363, t4721, t4726, t4731, t4735, t449);
        let (t4835, t4840, t4857) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1046::<F>(t1147, t1687, t1155, t1695, t3238, t3295, t3383, t3390, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773, t4776, t4779);
        let (t4858, t4861, t4862, t4865) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1047::<F>(t1156, t4857, t1694, t3403, t1155, t1129, t1138, t1148, t1157, t1683, t1695, t3327, t3332, t3357, t3371, t3376, t3401, t436, t4739, t4742, t4744, t4747, t4784, t4788, t4794, t4797, t4802, t4820, t4824, t4833, t4835, t4840);
    (t4820, t4823, t4824, t4832, t4833, t4835, t4840, t4857, t4858, t4861, t4862, t4865)
}
