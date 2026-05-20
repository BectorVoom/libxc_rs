//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2168/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2168<F: Float>(t43819: F, t11223: F, t699: F, t11205: F, t11208: F, t2403: F, t3298: F, t11220: F, t1114: F, t9709: F, t3304: F, t3301: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43820 = F::new(280.0) / F::new(81.0) * t43819;
    let t43835 = t699 * t11223;
    let t43837 = t699 * t11205;
    let t43839 = t699 * t11208;
    let t43855 = t2403 * t3298;
    let t43857 = t699 * t11220;
    let t43859 = t9709 * t1114;
    let t43861 = t2403 * t3304;
    let t43863 = t2403 * t3301;
    (t43820, t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863)
}
