//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1412;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta366<F: Float>(t15567: F, t3068: F, t1244: F, t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1011: F, t15031: F, t1212: F, t1226: F, t4965: F, t4953: F, t1229: F, t3242: F, t13969: F, t4979: F, t3506: F, t4973: F, t1227: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15569, t15574, t15580, t15591) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1412::<F>(t15567, t3068, t1244, t11697, t4949, t3577, t3431, t4729, t1174, t1011, t15031, t1212);
        let (t15594, t15610, t15615, t15642, t15645) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1413::<F>(t1226, t4965, t11697, t4953, t3577, t1229, t3242, t13969, t4979, t3506, t4973, t1227);
    (t15569, t15574, t15580, t15591, t15594, t15610, t15615, t15642, t15645)
}
