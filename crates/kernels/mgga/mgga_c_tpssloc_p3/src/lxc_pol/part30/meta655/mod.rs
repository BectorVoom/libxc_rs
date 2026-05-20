//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta655<F: Float>(t7557: F, t82632: F, t25836: F, t3216: F, t11094: F, t7627: F, t28: F, t40772: F, t1649: F, t2752: F, t26012: F, t6505: F, t1437: F, t6509: F, t1863: F, t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t12571: F, t608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t89672, t89698, t89702, t89953, t89992, t90087) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071::<F>(t7557, t82632, t25836, t3216, t11094, t7627, t28, t40772, t1649, t2752, t26012, t6505);
        let (t90091, t90095, t90098, t90101, t90104, t90114) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2072::<F>(t1437, t6509, t1863, t1864, t4021, t1410, t9231, t2240, t3961, t3967, t12571, t608);
    (t89672, t89698, t89702, t89953, t89992, t90087, t90091, t90095, t90098, t90101, t90104, t90114)
}
