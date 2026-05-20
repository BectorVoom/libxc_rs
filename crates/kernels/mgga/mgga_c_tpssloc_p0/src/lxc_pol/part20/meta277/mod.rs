//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta277 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1452;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1453;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1454;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1455;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1456;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta277<F: Float>(t10294: F, t268: F, t271: F, t6546: F, t2798: F, t2807: F, t896: F, t2815: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t2394: F, t885: F, t2772: F, t690: F, t2777: F, t2781: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10542, t10544) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1452::<F>(t10294, t268, t271, t6546);
        let (t10545, t10547, t10550, t10553) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1453::<F>(t10544, t2798, t2807, t896, t2815, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10542);
        let t10556 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1454::<F>(t2394, t885);
        let t10558 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1455::<F>(t2772, t690);
        let t10560 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1456::<F>(t2777, t690);
        let t10562 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1457::<F>(t2781, t690);
    (t10542, t10544, t10545, t10547, t10550, t10553, t10556, t10558, t10560, t10562)
}
