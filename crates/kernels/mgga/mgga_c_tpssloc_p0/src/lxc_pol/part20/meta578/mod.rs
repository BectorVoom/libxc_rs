//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2142;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta578<F: Float>(t10870: F, t3117: F, t1020: F, t10858: F, t248: F, t3101: F, t10961: F, t3108: F, t10423: F, t10937: F, t2955: F, t3158: F, t10383: F, t964: F, t10508: F, t3121: F, t10949: F, t11002: F, t1036: F, t10361: F, t10390: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43114, t43118, t43120, t43143, t43155) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2142::<F>(t10870, t3117, t1020, t10858, t248, t3101, t10961, t3108, t10423, t10937, t2955, t3158);
        let (t43157, t43161, t43167, t43176, t43186) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2143::<F>(t10383, t964, t1020, t10508, t248, t3121, t10949, t11002, t1036, t10361, t10390, t10423);
    (t43114, t43118, t43120, t43143, t43155, t43157, t43161, t43167, t43176, t43186)
}
