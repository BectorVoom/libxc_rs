//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2546;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta711<F: Float>(t10423: F, t13995: F, t10413: F, t10422: F, t14221: F, t10949: F, t14025: F, t10883: F, t13969: F, t14106: F, t13559: F, t2970: F, t973: F, t1036: F, t13942: F, t3047: F, t4616: F, t10890: F, t14507: F, t1041: F, t14188: F, t1020: F, t14489: F, t248: F, t3101: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49697, t49702, t49716, t49721, t49732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2546::<F>(t10423, t13995, t10413, t10422, t14221, t10949, t14025, t10883, t13969, t14106, t13559, t2970, t973);
        let (t49734, t49740, t49743, t49748, t49757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547::<F>(t1036, t13942, t3047, t4616, t10890, t14507, t1041, t13969, t14188, t1020, t14489, t248, t3101);
    (t49697, t49702, t49716, t49721, t49732, t49734, t49740, t49743, t49748, t49757)
}
