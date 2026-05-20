//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1774;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1775;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta443<F: Float>(t22927: F, t6897: F, t22666: F, t6891: F, t6888: F, t225: F, t3886: F, t3888: F, t6889: F, t1985: F, t6883: F, t6903: F, t22870: F, t539: F, t12033: F, t1375: F, t2016: F, t22688: F, t22905: F, t22908: F, t22910: F, t22913: F, t22918: F, t22922: F, t22924: F, t22926: F, t3758: F, t3889: F, t568: F, t6958: F, t6963: F, t6993: F, t22680: F, t533: F, t1390: F, t1983: F, t2379: F, t25: F, t1914: F, t193: F, t201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22928, t22930, t22931, t22934, t22935, t22936, t22940) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1774::<F>(t22927, t6897, t22666, t6891, t6888, t225, t3886, t3888, t6889, t1985, t6883, t6903);
        let (t22942, t22946) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1775::<F>(t22940, t22870, t539, t12033, t1375, t2016, t22688, t22905, t22908, t22910, t22913, t22918, t22922, t22924, t22926, t22928, t22931, t22936, t3758, t3889, t568, t6958, t6963, t6993);
        let (t22947, t22948, t22949, t22950, t22951, t22959) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1776::<F>(t22680, t22946, t533, t1390, t1983, t2379, t25, t1914, t193, t201);
    (t22928, t22930, t22934, t22935, t22940, t22942, t22947, t22948, t22949, t22950, t22951, t22959)
}
