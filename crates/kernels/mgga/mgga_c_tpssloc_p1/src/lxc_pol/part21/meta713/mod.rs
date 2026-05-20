//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2550;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta713<F: Float>(t13965: F, t3109: F, t1041: F, t13969: F, t14173: F, t247: F, t677: F, t4589: F, t10969: F, t41687: F, t1009: F, t13939: F, t1011: F, t1019: F, t10868: F, t248: F, t4347: F) -> (F, F, F, F, F, F, F, F) {
        let (t49831, t49846, t49850) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2550::<F>(t13965, t3109, t1041, t13969, t14173, t247, t677);
        let (t49852, t49854, t49864, t49866, t49871) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551::<F>(t1041, t4589, t49850, t10969, t41687, t1009, t13939, t1011, t1019, t10868, t248, t4347);
    (t49831, t49846, t49850, t49852, t49854, t49864, t49866, t49871)
}
