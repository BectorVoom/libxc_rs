//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta681<F: Float>(t10937: F, t18041: F, t1041: F, t13969: F, t17636: F, t17642: F, t17906: F, t3117: F, t17624: F, t2960: F, t5884: F, t698: F, t973: F, t5889: F, t10422: F, t17676: F, t3070: F, t17171: F, t2970: F, t17167: F, t10231: F, t17157: F, t17161: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t62499, t62510, t62515, t62534, t62556, t62559) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245::<F>(t10937, t18041, t1041, t13969, t17636, t17642, t17906, t3117, t17624, t2960, t5884, t698, t973);
        let (t62565, t62602, t62631, t62640, t62657, t62660) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246::<F>(t5889, t698, t973, t10422, t17676, t3070, t17171, t2970, t17167, t10231, t17157, t17161);
    (t62499, t62510, t62515, t62534, t62556, t62559, t62565, t62602, t62631, t62640, t62657, t62660)
}
