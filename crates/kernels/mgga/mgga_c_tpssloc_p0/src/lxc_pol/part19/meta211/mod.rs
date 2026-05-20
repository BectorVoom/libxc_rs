//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta211 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk892;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk893;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk894;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk895;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk896;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta211<F: Float>(t2906: F, t950: F, t10523: F, t2932: F, t959: F, t10195: F, t2768: F, t123: F, t10250: F, t882: F, t10294: F, t268: F, t271: F, t6546: F, t2798: F, t2807: F, t896: F, t2815: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t2394: F, t885: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t10524 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk892::<F>(t2906, t950);
        let (t10526, t10528, t10529, t10530) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk893::<F>(t10523, t10524, t2932, t959, t10195, t2768, t123);
        let (t10537, t10538) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk894::<F>(t10250, t882, t123);
        let (t10542, t10544) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk895::<F>(t10294, t268, t271, t6546);
        let (t10547, t10550, t10553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk896::<F>(t10544, t2798, t2807, t896, t2815, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10542);
        let t10556 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk897::<F>(t2394, t885);
    (t10524, t10526, t10528, t10529, t10530, t10537, t10538, t10544, t10547, t10550, t10553, t10556)
}
