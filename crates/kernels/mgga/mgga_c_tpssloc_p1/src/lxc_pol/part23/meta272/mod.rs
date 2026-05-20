//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk952;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta272<F: Float>(t20553: F, t550: F, t1343: F, t820: F, t1799: F, t6347: F, t3870: F, t20489: F, t20416: F, t210: F, t214: F, t20356: F, t221: F, t5196: F, t12188: F, t12194: F, t12196: F, t12215: F, t12236: F, t1315: F, t16078: F, t16108: F, t16119: F, t19768: F, t19776: F, t19779: F, t19791: F, t5195: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk952::<F>(t20553, t550, t1343, t820, t1799, t6347, t3870, t20489, t20416, t210, t214, t20356);
        let (t20586, t20594) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk953::<F>(t221, t5196, t6347, t12188, t12194, t12196, t12215, t12236, t1315, t16078, t16108, t16119, t19768, t19776, t19779, t19791, t20576, t20582, t5195);
    (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582, t20586, t20594)
}
