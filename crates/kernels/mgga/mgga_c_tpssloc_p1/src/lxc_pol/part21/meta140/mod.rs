//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta140 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk924;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk925;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk926;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk927;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk928;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta140<F: Float>(t3333: F, t3359: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F, t449: F, t1143: F, t1147: F, t1146: F, t445: F, t440: F, t1155: F, t1156: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3360, t3363, t3368) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk924::<F>(t3333, t3359, t3236, t3238, t3245, t3250, t3254);
        let (t3369, t3371) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk925::<F>(t3368, t449, t1143, t1147);
        let (t3374, t3375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk926::<F>(t1146, t445);
        let t3376 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk927::<F>(t3375, t440);
        let t3377 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk928::<F>(t1155);
        let t3378 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk929::<F>(t1156, t3377);
    (t3360, t3363, t3368, t3369, t3371, t3374, t3375, t3376, t3377, t3378)
}
