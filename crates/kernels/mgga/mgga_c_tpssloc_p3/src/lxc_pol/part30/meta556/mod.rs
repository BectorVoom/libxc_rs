//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1915;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta556<F: Float>(t28205: F, t6889: F, t1985: F, t6347: F, t6890: F, t6888: F, t26193: F, t7691: F, t1842: F, t7749: F, t3887: F, t2015: F, t6439: F, t12021: F, t22933: F, t1375: F, t1843: F, t20060: F, t2016: F, t22924: F, t22926: F, t26366: F, t26475: F, t27067: F, t28193: F, t28196: F, t28201: F, t5321: F, t6440: F, t6958: F, t7729: F, t7750: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28206, t28207, t28209, t28210, t28211, t28213, t28214, t28220, t28223) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1915::<F>(t28205, t6889, t1985, t6347, t6890, t6888, t26193, t7691, t1842, t7749, t3887, t2015, t6439);
        let (t28224, t28232, t28233, t28236) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1916::<F>(t12021, t28223, t22933, t6439, t6889, t1985, t1375, t1843, t20060, t2016, t22924, t22926, t26366, t26475, t27067, t28193, t28196, t28201, t28207, t28211, t28214, t28220, t5321, t6440, t6958, t7729, t7750);
    (t28206, t28209, t28210, t28213, t28220, t28224, t28232, t28233, t28236)
}
