//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1346;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta460<F: Float>(t42110: F, t42113: F, t76637: F, t959: F, t17934: F, t5804: F, t5694: F, t42100: F, t42102: F, t5695: F, t60357: F, t21268: F, t49489: F, t10702: F, t2844: F, t1557: F, t68924: F, t17195: F, t5727: F, t5730: F, t59959: F, t21300: F, t4354: F, t1637: F, t4700: F, t68711: F, t76634: F, t76636: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t76641, t76643, t76644, t76647, t76652, t76654) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1346::<F>(t42110, t42113, t76637, t959, t17934, t5804, t5694, t42100, t42102, t5695, t60357, t21268, t49489);
        let (t76657, t76659, t76661, t76663, t76665, t76666) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347::<F>(t10702, t2844, t76644, t1557, t68924, t17195, t5727, t5730, t59959, t21300, t4354, t1637, t4700, t68711, t76634, t76636, t76641, t76643, t76647, t76652, t76654);
    (t76641, t76643, t76644, t76647, t76652, t76654, t76657, t76659, t76661, t76663, t76665, t76666)
}
