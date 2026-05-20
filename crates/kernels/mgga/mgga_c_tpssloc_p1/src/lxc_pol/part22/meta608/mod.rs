//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta608<F: Float>(t10470: F, t11058: F, t381: F, t1615: F, t6739: F, t11064: F, t3199: F, t49649: F, t11045: F, t10164: F, t1634: F, t11190: F, t1670: F, t3242: F, t457: F, t2394: F, t4734: F, t1654: F, t9698: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50508, t50509, t50516, t50592, t50610, t50628, t50819) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134::<F>(t10470, t11058, t381, t1615, t6739, t11064, t3199, t49649, t11045, t10164, t1634, t11190, t1670);
        let (t50822, t50826, t50827, t50834) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2135::<F>(t3242, t457, t2394, t4734, t1654, t9698);
    (t50508, t50509, t50516, t50592, t50610, t50628, t50819, t50822, t50826, t50827, t50834)
}
