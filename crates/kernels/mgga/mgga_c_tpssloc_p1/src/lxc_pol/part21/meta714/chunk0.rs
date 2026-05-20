//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2552/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2552<F: Float>(t14134: F, t3117: F, t10863: F, t4571: F, t13969: F, t14102: F, t3039: F, t10876: F, t13990: F, t3048: F, t14137: F, t10952: F, t13970: F) -> (F, F, F, F, F, F, F) {
    let t49873 = t3117 * t14134;
    let t49877 = t10863 * t4571;
    let t49884 = t3039 * t13969 * t14102;
    let t49887 = t10876 * t13969 * t13990;
    let t49889 = t3048 * t14134;
    let t49892 = t3048 * t14137;
    let t49894 = t10952 * t13970;
    (t49873, t49877, t49884, t49887, t49889, t49892, t49894)
}
