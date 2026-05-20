//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta411<F: Float>(t1174: F, t6177: F, t698: F, t3545: F, t6109: F, t15753: F, t4889: F, t1244: F, t3068: F, t478: F, t6163: F, t6183: F, t22430: F, t580: F, t111: F, t20292: F, t172: F, t20742: F, t763: F, t21066: F, t870: F, t2752: F, t20767: F, t751: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66057, t66500, t66545, t66622, t66668) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227::<F>(t1174, t6177, t698, t3545, t6109, t15753, t4889, t1244, t3068, t478, t6163, t6183);
        let (t67000, t67001, t67099, t67112, t67154, t67159) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1228::<F>(t22430, t580, t111, t20292, t172, t20742, t763, t21066, t870, t2752, t20767, t751);
    (t66057, t66500, t66545, t66622, t66668, t67000, t67001, t67099, t67112, t67154, t67159)
}
