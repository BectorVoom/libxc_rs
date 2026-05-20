//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk860;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk861;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk862;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk863;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta132<F: Float>(t3040: F, t381: F, t1932: F, t3131: F, t1022: F, t1049: F, t1060: F, t3120: F, t1014: F, t3032: F, t3031: F, t360: F, t3166: F, t383: F, t1003: F, t1058: F, t1061: F, t1063: F, t3076: F, t3180: F, t3186: F, t353: F, t384: F, t1055: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3187, t3188) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk860::<F>(t3040, t381, t1932, t3131);
        let (t3189, t3193, t3196, t3197, t3199) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk861::<F>(t3187, t3188, t1022, t1049, t1060, t3120, t381, t1014, t3032);
        let t3200 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk862::<F>(t3031, t3199);
        let (t3201, t3202, t3204, t3206) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk863::<F>(t1932, t360, t3187, t3166, t383, t1003, t1058, t1061, t1063, t3076, t3180, t3186, t3189, t3193, t3197, t3200, t353, t384);
        let t3207 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk864::<F>(t1055, t3206);
    (t3188, t3189, t3193, t3196, t3197, t3199, t3200, t3201, t3202, t3204, t3206, t3207)
}
