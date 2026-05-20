//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta314<F: Float>(t11399: F, t1147: F, t1156: F, t1164: F, t3411: F, t3419: F, t3423: F, t11203: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11230: F, t11233: F) -> (F, F, F, F, F, F) {
        let (t11478, t11480, t11482, t11484, t11487, t11496) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1575::<F>(t11399, t1147, t1156, t1164, t3411, t3419, t3423, t11203, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224, t11230, t11233);
    (t11478, t11480, t11482, t11484, t11487, t11496)
}
