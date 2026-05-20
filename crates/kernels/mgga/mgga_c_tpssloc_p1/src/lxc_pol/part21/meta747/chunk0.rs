//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2618/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618<F: Float>(t3566: F, t5023: F, t15734: F, t3490: F, t11789: F, t1227: F, t248: F, t4733: F, t11712: F, t11913: F, t491: F, t11887: F, t52834: F) -> (F, F, F, F, F) {
    let t53507 = t3566 * t5023;
    let t53515 = t3490 * t15734;
    let t53519 = t1227 * t248 * t11789 * t4733;
    let t53545 = t11712 * t11913 * t491;
    let t53565 = t52834 * t11887;
    (t53507, t53515, t53519, t53545, t53565)
}
