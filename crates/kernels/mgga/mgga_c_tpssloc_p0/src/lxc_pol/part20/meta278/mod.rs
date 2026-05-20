//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta278 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1458;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1459;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1460;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1461;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta278<F: Float>(t154: F, t3061: F, t10305: F, t123: F, t10309: F, t2768: F, t10316: F, t882: F, t10321: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t894: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10564, t10565, t10566) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1458::<F>(t154, t3061, t10305, t123);
        let (t10568, t10569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1459::<F>(t10309, t2768, t123);
        let (t10571, t10572) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1460::<F>(t10316, t882, t123);
        let (t10574, t10575) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1461::<F>(t10321, t882, t123);
        let (t10577, t10588, t10589) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1462::<F>(t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t894);
    (t10564, t10565, t10566, t10568, t10569, t10571, t10572, t10574, t10575, t10577, t10588, t10589)
}
