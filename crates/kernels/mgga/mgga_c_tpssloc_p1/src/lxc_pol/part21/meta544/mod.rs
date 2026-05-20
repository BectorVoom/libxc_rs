//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta544<F: Float>(t248: F, t3521: F, t5975: F, t1227: F, t1409: F, t15701: F, t15700: F, t3578: F, t1735: F, t4729: F, t18232: F, t4900: F) -> (F, F, F, F, F, F, F, F) {
        let (t18392, t18393, t18395, t18396, t18397, t18400, t18401, t18404) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2231::<F>(t248, t3521, t5975, t1227, t1409, t15701, t15700, t3578, t1735, t4729, t18232, t4900);
    (t18392, t18393, t18395, t18396, t18397, t18400, t18401, t18404)
}
