//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2556/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2556<F: Float>(t1041: F, t4584: F, t49850: F, t10422: F, t14032: F, t3070: F, t13969: F, t14166: F, t14159: F, t2960: F, t14146: F, t14068: F) -> (F, F, F, F, F, F) {
    let t50047 = t1041 * t49850 * t4584;
    let t50056 = t3070 * t10422 * t14032;
    let t50062 = t1041 * t13969 * t14166;
    let t50077 = t2960 * t14159;
    let t50084 = t1041 * t13969 * t14146;
    let t50094 = t3070 * t10422 * t14068;
    (t50047, t50056, t50062, t50077, t50084, t50094)
}
