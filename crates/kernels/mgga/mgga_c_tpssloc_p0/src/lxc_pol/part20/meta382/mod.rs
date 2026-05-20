//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1742;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1743;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1744;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta382<F: Float>(t776: F, t828: F, t13228: F, t13222: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t4180: F, t4182: F, t4181: F, t9632: F, t2642: F, t4166: F, t2617: F, t4177: F, t2628: F, t836: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13229, t13231, t13234, t13237, t13242) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1742::<F>(t776, t828, t13228, t13222, t1500, t2693, t4163, t838, t120, t4233);
        let (t13244, t13248, t13251) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1743::<F>(t13242, t4180, t4182, t4181, t9632, t2642, t4166);
        let t13254 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1744::<F>(t2617, t4177);
        let (t13257, t13258) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1745::<F>(t2628, t836, t812);
    (t13229, t13231, t13234, t13237, t13242, t13244, t13248, t13251, t13254, t13257, t13258)
}
