//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta298<F: Float>(t3120: F, t3188: F, t1059: F, t10471: F, t10474: F, t10470: F, t10482: F, t6739: F, t11047: F, t3127: F, t3131: F, t1049: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11054, t11055, t11058, t11059, t11060, t11061, t11064, t11065, t11066, t11067, t11077) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1515::<F>(t3120, t3188, t1059, t10471, t10474, t10470, t10482, t6739, t11047, t3127, t3131, t1049);
    (t11054, t11055, t11058, t11059, t11060, t11061, t11064, t11065, t11066, t11067, t11077)
}
