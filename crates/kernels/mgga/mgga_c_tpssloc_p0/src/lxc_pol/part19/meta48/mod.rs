//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta48 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk319;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk320;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk321;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk322;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk323;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta48<F: Float>(t134: F, t340: F, t344: F, t221: F, t339: F, t209: F, t338: F, t39: F, t119: F, t60: F, t270: F, t271: F, t883: F, t607: F, t906: F, t910: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t967, t969, t971, t972) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk319::<F>(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk320::<F>(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk321::<F>(t119, t60);
        let t976 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk322::<F>(t270, t271);
        let t977 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk323::<F>(t974, t976);
        let (t978, t979, t980, t984) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk324::<F>(t344, t883, t607, t977, t906, t910);
    (t967, t969, t971, t972, t973, t974, t976, t977, t978, t979, t980, t984)
}
