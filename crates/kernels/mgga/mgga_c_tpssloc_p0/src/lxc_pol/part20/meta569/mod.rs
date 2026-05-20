//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2130;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta569<F: Float>(t10508: F, t248: F, t3039: F, t3041: F, t3020: F, t3030: F, t3032: F, t3038: F, t10360: F, t1040: F, t1043: F, t204: F, t1041: F, t884: F, t1009: F, t10358: F, t1011: F, t1019: F, t10283: F, t969: F, t10189: F, t3014: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42735, t42741, t42742, t42743, t42746, t42749) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2130::<F>(t10508, t248, t3039, t3041, t3020, t3030, t3032, t3038, t10360, t1040, t1043, t204);
        let (t42752, t42754, t42756, t42762, t42771) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2131::<F>(t1041, t248, t42749, t884, t1009, t10358, t1011, t1019, t10283, t969, t10189, t3014);
    (t42735, t42741, t42742, t42743, t42746, t42749, t42752, t42754, t42756, t42762, t42771)
}
