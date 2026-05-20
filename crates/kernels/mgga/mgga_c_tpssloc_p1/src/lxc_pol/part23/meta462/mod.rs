//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta462<F: Float>(t136: F, t2826: F, t76597: F, t76593: F, t41880: F, t76572: F, t76576: F, t908: F, t76589: F, t10304: F, t76581: F, t76585: F, t68500: F, t68502: F, t68504: F, t68506: F, t76624: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t76877, t76880, t76887, t76890, t76893, t76896, t76899) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352::<F>(t136, t2826, t76597, t76593, t41880, t76572, t76576, t908, t76589, t10304, t76581, t76585);
        let (t76901, t76903) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353::<F>(t68500, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t76893, t76896, t76899, t136, t76624, t908);
    (t76877, t76880, t76887, t76890, t76893, t76896, t76899, t76901, t76903)
}
