//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta323<F: Float>(t1090: F, t11789: F, t248: F, t1227: F, t486: F, t676: F, t1216: F, t1213: F, t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t10471: F, t11715: F, t11712: F, t11721: F, t6739: F, t3502: F, t3508: F, t11707: F, t3609: F, t3623: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11792, t11818, t11821, t11834, t11835) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1348::<F>(t1090, t11789, t248, t1227, t486, t676, t1216, t1213, t11552, t221, t456, t1197, t698);
        let (t11836, t11881, t11883, t11888, t11889, t11904, t11907) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1349::<F>(t1174, t11835, t10471, t11715, t11712, t11721, t6739, t3502, t3508, t11707, t3609, t3623);
    (t11792, t11818, t11821, t11834, t11836, t11881, t11883, t11888, t11889, t11904, t11907)
}
