//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1909;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta604<F: Float>(t22716: F, t7701: F, t1834: F, t212: F, t22642: F, t6890: F, t1373: F, t254: F, t26215: F, t81228: F, t81326: F, t16436: F, t1985: F, t6889: F, t6906: F, t6897: F, t6907: F, t90544: F, t22662: F, t26193: F, t26203: F, t6883: F, t7700: F, t80645: F, t22633: F, t22635: F, t26214: F, t3719: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90659, t90663, t90665, t90686, t90690) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1909::<F>(t22716, t7701, t1834, t212, t22642, t6890, t1373, t254, t26215, t81228, t81326, t16436, t1985, t6889, t6906);
        let (t90701, t90704, t90707, t90723, t90728) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1910::<F>(t6897, t6907, t90544, t1985, t22662, t26193, t26203, t6883, t7700, t80645, t22633, t22635, t26214, t3719);
    (t90659, t90663, t90665, t90686, t90690, t90701, t90704, t90707, t90723, t90728)
}
