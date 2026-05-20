//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1526;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta334<F: Float>(t3788: F, t836: F, t1336: F, t5252: F, t3777: F, t5245: F, t1834: F, t3787: F, t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t112: F, t5363: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16397, t16398) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1526::<F>(t3788, t836, t1336);
        let (t16400, t16401, t16428, t16439, t16460, t16497, t16521) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527::<F>(t16398, t5252, t3777, t5245, t1834, t3787, t225, t5319, t5217, t1390, t5356, t112, t5363);
    (t16397, t16398, t16400, t16401, t16428, t16439, t16460, t16497, t16521)
}
