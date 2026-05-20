//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1396;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1397;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta259<F: Float>(t225: F, t3023: F, t1053: F, t68: F, t1065: F, t3175: F, t3021: F, t3206: F, t3174: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10160, t10163, t10164, t10165, t10166, t10167, t10170, t10181, t10182, t10186) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1396::<F>(t225, t3023, t1053, t68, t1065, t3175, t3021, t3206, t3174, t1887, t337, t615);
        let t10189 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1397::<F>(t134, t976);
        let t10190 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1398::<F>(t10189, t984);
    (t10160, t10163, t10164, t10165, t10166, t10167, t10170, t10181, t10182, t10186, t10189, t10190)
}
