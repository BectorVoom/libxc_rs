//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta226<F: Float>(t15908: F, t2375: F, t1787: F, t2516: F, t17: F, t2663: F, t5157: F, t1788: F, t2225: F, t2221: F, t2223: F, t12248: F, t68: F) -> (F, F, F, F, F, F, F, F) {
        let (t15909, t15971, t15972, t15979, t15982, t15984, t15986, t16046) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk874::<F>(t15908, t2375, t1787, t2516, t17, t2663, t5157, t1788, t2225, t2221, t2223, t12248, t68);
    (t15909, t15971, t15972, t15979, t15982, t15984, t15986, t16046)
}
