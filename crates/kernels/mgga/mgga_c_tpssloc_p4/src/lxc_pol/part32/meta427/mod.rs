//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1653;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta427<F: Float>(t17: F, t19573: F, t6320: F, t750: F, t1388: F, t1799: F, t15877: F, t11979: F, t15890: F, t15895: F, t588: F, t6328: F, t592: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t6347: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19591) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1653::<F>(t17, t19573, t6320, t750, t1388, t1799, t15877, t11979, t15890, t15895, t588, t6328);
        let (t19592, t19594, t19595) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1654::<F>(t19591, t592, t6328, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19577, t19581, t19588, t19589, t19590, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t6347, t9457, t9476, t9484);
    (t19574, t19576, t19577, t19581, t19588, t19589, t19590, t19592, t19594, t19595)
}
