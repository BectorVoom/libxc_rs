//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2264;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta560<F: Float>(t15453: F, t17686: F, t4582: F, t17635: F, t4972: F, t1090: F, t6230: F, t3578: F, t6219: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F, t5019: F, t4993: F, t5005: F, t1202: F, t6164: F, t5024: F, t11692: F, t11792: F, t11821: F, t15671: F, t15691: F, t15699: F, t15740: F, t3577: F, t488: F, t4950: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2264::<F>(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let (t18982, t18989) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2265::<F>(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
    (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18975, t18982, t18989)
}
