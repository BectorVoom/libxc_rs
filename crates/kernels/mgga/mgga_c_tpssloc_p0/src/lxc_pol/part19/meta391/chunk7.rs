//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1478/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1478<F: Float>(t11153: F, t1176: F, t11881: F, t45113: F, t11773: F, t1227: F, t13969: F, t11168: F, t1174: F, t3431: F, t3540: F, t3567: F) -> (F, F, F, F, F) {
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    let t45211 = t1227 * t13969 * t11773;
    let t45222 = t1174 * t3431 * t11168;
    let t45224 = t3567 * t3540;
    (t45192, t45197, t45211, t45222, t45224)
}
