//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta211 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1293;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1294;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1295;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1296;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta211<F: Float>(t5154: F, t763: F, t1787: F, t67: F, t758: F, t193: F, t533: F, t1845: F, t3701: F, t3692: F, t1307: F, t1388: F, t2408: F, t2417: F, t2423: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t3918: F, t5122: F, t5126: F, t5127: F, t5131: F, t5153: F, t184: F, t5151: F, t17: F, t750: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5155, t5156, t5157, t5158, t5159, t5160) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1293::<F>(t5154, t763, t1787, t67, t758, t193, t533);
        let t5161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1294::<F>(t1845, t3701);
        let (t5164, t5165) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1295::<F>(t3692, t1307, t1388, t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t3918, t5122, t5126, t5127, t5131, t5153, t5156, t5159, t5160, t5161);
        let t5166 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1296::<F>(t184, t5151);
        let (t5167, t5168) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1297::<F>(t17, t5166, t1787, t750);
    (t5155, t5156, t5157, t5158, t5159, t5160, t5161, t5164, t5165, t5166, t5167, t5168)
}
