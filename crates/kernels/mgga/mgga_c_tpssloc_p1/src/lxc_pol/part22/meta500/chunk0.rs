//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1934/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1934<F: Float>(t1615: F, t5914: F, t1060: F, t21594: F, t381: F, t21390: F) -> (F, F, F, F, F) {
    let t21626 = t5914 * t1615;
    let t21627 = t21626 * t1060;
    let t21634 = t381 * t21594;
    let t21635 = t21634 * t1060;
    let t21637 = t381 * t21390;
    (t21626, t21627, t21634, t21635, t21637)
}
