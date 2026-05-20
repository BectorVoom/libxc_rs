//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1880;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1881;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta401<F: Float>(t14473: F, t961: F, t2948: F, t4483: F, t14364: F, t300: F, t2907: F, t4496: F, t959: F, t2952: F, t10623: F, t1589: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14472: F, t14238: F) -> (F, F, F, F, F, F, F, F) {
        let (t14475, t14477, t14479, t14480, t14482, t14484, t14486) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1880::<F>(t14473, t961, t2948, t4483, t14364, t300, t2907, t4496, t959, t2952, t10623, t1589);
        let t14487 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1881::<F>(t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479, t14482, t14484, t14486);
        let t14488 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1882::<F>(t14238, t14487);
    (t14475, t14477, t14479, t14480, t14482, t14484, t14486, t14488)
}
