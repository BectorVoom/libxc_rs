//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2148;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta581<F: Float>(t10883: F, t10884: F, t248: F, t3101: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F, t3131: F, t3047: F, t3077: F, t10908: F, t3114: F, t1036: F, t10438: F, t221: F, t339: F, t42813: F, t10283: F, t995: F, t10931: F, t135: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43285, t43288, t43291, t43292, t43298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2148::<F>(t10883, t10884, t248, t3101, t10473, t361, t363, t42342, t42345, t3131, t3047, t3077);
        let (t43301, t43303, t43307, t43310, t43313) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149::<F>(t10908, t3114, t1036, t10438, t221, t339, t42813, t10283, t995, t10931, t135, t973);
    (t43285, t43288, t43291, t43292, t43298, t43301, t43303, t43307, t43310, t43313)
}
