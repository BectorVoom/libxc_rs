//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1088/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1088<F: Float>(t4211: F, t9874: F, t1472: F, t9862: F, t1519: F, t9971: F, t1496: F, t41083: F, t1516: F, t40965: F, t4166: F, t9637: F, t12985: F, t9577: F, t41189: F, t4134: F) -> (F, F, F, F, F, F, F, F) {
    let t46433 = t4211 * t9874;
    let t46439 = t1472 * t9862;
    let t46524 = t9971 * t1519;
    let t46546 = t41083 * t1496;
    let t46577 = t40965 * t1516;
    let t46657 = t4166 * t9637;
    let t46764 = t9577 * t12985;
    let t46772 = t41189 * t4134;
    (t46433, t46439, t46524, t46546, t46577, t46657, t46764, t46772)
}
