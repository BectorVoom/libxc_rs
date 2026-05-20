//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1139;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1140;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta193<F: Float>(t5685: F, t882: F, t123: F, t2765: F, t4335: F, t5679: F, t5683: F, t291: F, t1557: F, t4354: F, t1556: F, t913: F, t2792: F, t1547: F, t2798: F, t2802: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5686, t5687, t5689, t5691, t5693, t5694, t5695) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1139::<F>(t5685, t882, t123, t2765, t4335, t5679, t5683, t291, t1557, t4354, t1556, t913);
        let (t5697, t5698) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1140::<F>(t2792, t5695, t1547);
        let (t5699, t5705) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1141::<F>(t2798, t5698, t2802, t4335, t5679, t5683, t5687);
    (t5686, t5687, t5689, t5691, t5693, t5694, t5695, t5697, t5698, t5699, t5705)
}
