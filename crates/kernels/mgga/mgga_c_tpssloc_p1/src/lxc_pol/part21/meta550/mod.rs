//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta550<F: Float>(t18583: F, t3578: F, t17691: F, t4972: F, t4582: F, t15615: F, t17686: F, t1155: F, t6069: F, t1695: F, t4857: F, t6088: F) -> (F, F, F, F, F, F, F, F) {
        let (t18584, t18589, t18590, t18593, t18594, t18603, t18606, t18609) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2243::<F>(t18583, t3578, t17691, t4972, t4582, t15615, t17686, t1155, t6069, t1695, t4857, t6088);
    (t18584, t18589, t18590, t18593, t18594, t18603, t18606, t18609)
}
