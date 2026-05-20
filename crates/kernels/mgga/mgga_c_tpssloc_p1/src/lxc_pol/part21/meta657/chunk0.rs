//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2458/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2458<F: Float>(t22715: F, t268: F, t405: F, t2403: F, t3298: F, t1114: F, t9709: F, t3304: F, t3301: F, t39267: F, t404: F, t410: F) -> (F, F, F, F, F, F, F) {
    let t43819 = t268 * t22715 * t405;
    let t43820 = F::new(280.0) / F::new(81.0) * t43819;
    let t43855 = t2403 * t3298;
    let t43859 = t9709 * t1114;
    let t43861 = t2403 * t3304;
    let t43863 = t2403 * t3301;
    let t43880 = F::new(1.0) / t410 / t39267 / t404 / F::new(96.0);
    (t43819, t43820, t43855, t43859, t43861, t43863, t43880)
}
