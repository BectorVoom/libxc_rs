//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1595;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta289<F: Float>(t248: F, t2776: F, t3051: F, t1041: F, t3103: F, t3109: F, t3114: F, t376: F, t676: F, t1023: F, t1020: F, t1017: F, t3087: F, t1015: F, t1012: F, t2928: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10489, t10490, t10496, t10504, t10508) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1595::<F>(t248, t2776, t3051, t1041, t3103, t3109, t3114, t376, t676);
        let (t10510, t10511, t10516, t10517, t10523) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1596::<F>(t1023, t10508, t248, t1020, t1017, t3087, t1015, t1012, t2928, t320);
    (t10489, t10490, t10496, t10504, t10508, t10510, t10511, t10516, t10517, t10523)
}
