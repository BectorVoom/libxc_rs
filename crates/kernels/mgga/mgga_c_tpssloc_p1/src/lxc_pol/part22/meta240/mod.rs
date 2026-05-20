//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1327;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1328;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta240<F: Float>(t252: F, t9971: F, t856: F, t68: F, t261: F, t2751: F, t1053: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t271: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10080, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1327::<F>(t252, t9971, t856, t68, t261, t2751, t1053, t1887, t337, t615);
        let t10189 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1328::<F>(t134, t976);
        let (t10190, t10213) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1329::<F>(t10189, t984, t271, t2775);
    (t10080, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186, t10189, t10190, t10213)
}
