//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1326;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1327;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta233<F: Float>(t205: F, t9558: F, t210: F, t214: F, t9458: F, t213: F, t776: F, t221: F, t2553: F, t59: F, t8705: F, t207: F, t215: F, t2570: F, t782: F, t2573: F, t2690: F, t154: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9559, t9561, t9566, t9569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1326::<F>(t205, t9558, t210, t214, t9458, t213, t776, t221, t2553, t59, t8705);
        let (t9572, t9573) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1327::<F>(t207, t215, t9569, t2570, t782);
        let (t9574, t9576, t9577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1328::<F>(t2573, t9573, t2690, t59, t154);
    (t9559, t9561, t9566, t9569, t9572, t9573, t9574, t9576, t9577)
}
