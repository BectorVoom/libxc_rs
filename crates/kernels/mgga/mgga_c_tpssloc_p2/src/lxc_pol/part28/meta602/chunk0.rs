//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1905/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1905<F: Float>(t22633: F, t22637: F, t90566: F, t26331: F, t26333: F, t80650: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F) -> (F, F, F, F, F) {
    let t90568 = t22633 * t90566 * t22637;
    let t90571 = t26331 * t80650 * t26333;
    let t90582 = t22724 * t26474;
    let t90584 = t22751 * t26194;
    let t90591 = t80830 * t1887;
    (t90568, t90571, t90582, t90584, t90591)
}
