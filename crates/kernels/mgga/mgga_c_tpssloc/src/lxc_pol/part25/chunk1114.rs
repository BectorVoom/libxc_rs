//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1114/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1114<F: Float>(t1992: F, t40475: F, t550: F, t6976: F, t22897: F, t3792: F, t81028: F, t22899: F, t6914: F, t22715: F, t6887: F, t6970: F) -> (F, F, F, F, F) {
    let t81177 = t1992 * t6976 * t40475 * t550;
    let t81181 = t1992 * t22897 * t81028 * t3792;
    let t81184 = t6914 * t22899;
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    (t81177, t81181, t81184, t81186, t81187)
}
