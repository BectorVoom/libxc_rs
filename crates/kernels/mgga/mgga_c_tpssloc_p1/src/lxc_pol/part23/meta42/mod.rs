//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk284;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk285;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk286;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk287;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk288;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk289;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta42<F: Float>(t315: F, t942: F, t880: F, t906: F, t323: F, t300: F, t134: F, t340: F, t344: F, t221: F, t339: F, t209: F, t338: F, t39: F, t119: F, t60: F, t270: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t943, t945, t948, t951) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk284::<F>(t315, t942, t880, t906, t323);
        let t959 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk285::<F>(t300, t315);
        let (t967, t971, t972) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk286::<F>(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk287::<F>(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk288::<F>(t119, t60);
        let t976 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk289::<F>(t270, t271);
        let t977 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk290::<F>(t974, t976);
    (t943, t945, t948, t951, t959, t967, t971, t972, t973, t974, t976, t977)
}
