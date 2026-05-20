//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta808 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2823;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2824;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2825;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2826;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta808<F: Float>(t2: F, t4324: F, t584: F, t1534: F, t16: F, t17139: F, t14389: F, t48763: F, t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F, t47774: F, t47783: F, t55716: F, t2394: F, t5678: F, t17156: F, t2244: F, t123: F, t882: F, t17184: F, t690: F, t17179: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t59627, t59629, t59631, t59637, t59650) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2822::<F>(t2, t4324, t584, t1534, t16, t17139, t14389, t48763, t41656, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t47738);
        let t59655 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2823::<F>(t47774, t47783, t55716);
        let t59657 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2824::<F>(t2394, t5678);
        let (t59659, t59661) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2825::<F>(t17156, t2244, t123, t882);
        let t59663 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2826::<F>(t17184, t690);
        let t59665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2827::<F>(t17179, t690);
    (t59627, t59629, t59631, t59637, t59650, t59655, t59657, t59659, t59661, t59663, t59665)
}
