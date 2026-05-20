//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta635<F: Float>(t3862: F, t5231: F, t12328: F, t1815: F, t1336: F, t2691: F, t3788: F, t5252: F, t3787: F, t5318: F, t40041: F, t544: F, t68: F, t12020: F, t1842: F, t1307: F, t193: F, t111: F, t5363: F, t6470: F, t19530: F, t626: F, t1447: F, t2349: F, t2281: F, t5489: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54786, t54793, t54812, t54905, t54963) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172::<F>(t3862, t5231, t12328, t1815, t1336, t2691, t3788, t5252, t3787, t5318, t40041, t544, t68);
        let (t55118, t55224, t55353, t55388, t55420, t55491, t55531) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2173::<F>(t12020, t1842, t1307, t193, t111, t5363, t6470, t19530, t626, t1447, t2349, t2281, t5489);
    (t54786, t54793, t54812, t54905, t54963, t55118, t55224, t55353, t55388, t55420, t55491, t55531)
}
