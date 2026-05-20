//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta765<F: Float>(t16391: F, t16398: F, t12283: F, t16244: F, t3862: F, t5231: F, t16356: F, t3726: F, t12328: F, t1815: F, t16397: F, t3777: F, t5252: F, t1336: F, t2691: F, t3788: F, t16028: F, t225: F, t40041: F, t544: F, t68: F, t1332: F, t16046: F, t1338: F, t16413: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t54750, t54764, t54785, t54787, t54793, t54801) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642::<F>(t16391, t16398, t12283, t16244, t3862, t5231, t16356, t3726, t12328, t1815, t16397, t3777, t5252);
        let (t54811, t54825, t54963, t54976, t55039) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643::<F>(t1336, t2691, t3788, t5252, t16028, t225, t40041, t544, t68, t1332, t16046, t1338, t16413);
    (t54750, t54764, t54785, t54787, t54793, t54801, t54811, t54825, t54963, t54976, t55039)
}
