//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta559<F: Float>(t18914: F, t18939: F, t475: F, t1214: F, t248: F, t3508: F, t5011: F, t4977: F, t4582: F, t11692: F, t1174: F, t1213: F, t1227: F, t15610: F, t15642: F, t15645: F, t18393: F, t18397: F, t18401: F, t18574: F, t18577: F, t18580: F, t18584: F, t18590: F, t18594: F, t3506: F, t3577: F, t488: F, t4974: F, t4989: F, t5005: F, t5024: F) -> (F, F, F, F, F, F, F) {
        let (t18940, t18941, t18943, t18946, t18947, t18948, t18951) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2263::<F>(t18914, t18939, t475, t1214, t248, t3508, t5011, t4977, t4582, t11692, t1174, t1213, t1227, t15610, t15642, t15645, t18393, t18397, t18401, t18574, t18577, t18580, t18584, t18590, t18594, t3506, t3577, t488, t4974, t4989, t5005, t5024);
    (t18940, t18941, t18943, t18946, t18947, t18948, t18951)
}
