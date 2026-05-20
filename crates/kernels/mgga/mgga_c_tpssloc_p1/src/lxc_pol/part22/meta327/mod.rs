//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1513;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta327<F: Float>(t225: F, t5211: F, t1332: F, t5343: F, t1372: F, t1824: F, t5286: F, t562: F, t12248: F, t68: F, t544: F, t5333: F, t5230: F) -> (F, F, F, F, F, F, F, F) {
        let (t16030, t16033, t16036, t16040, t16046, t16047, t16055) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1513::<F>(t225, t5211, t1332, t5343, t1372, t1824, t5286, t562, t12248, t68, t544, t5333);
        let t16060 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1514::<F>(t5230, t68);
    (t16030, t16033, t16036, t16040, t16046, t16047, t16055, t16060)
}
