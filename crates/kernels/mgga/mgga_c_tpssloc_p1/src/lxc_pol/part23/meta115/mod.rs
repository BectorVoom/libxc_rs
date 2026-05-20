//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk596;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta115<F: Float>(t340: F, t974: F, t1604: F, t225: F, t1539: F, t248: F, t3051: F, t1041: F, t247: F, t375: F, t1043: F, t2775: F, t2770: F, t3061: F, t135: F, t1606: F, t973: F, t1036: F, t1612: F, t1616: F, t3101: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4546, t4557, t4571, t4572, t4582) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk596::<F>(t340, t974, t1604, t225, t1539, t248, t3051, t1041, t247, t375);
        let (t4583, t4588, t4604, t4625, t4630) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk597::<F>(t1043, t2775, t2770, t3061, t135, t1606, t973, t1036, t1612, t1616, t248, t3101);
    (t4546, t4557, t4571, t4572, t4582, t4583, t4588, t4604, t4625, t4630)
}
