//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1899/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1899<F: Float>(t22751: F, t26190: F, t26356: F, t6914: F, t1385: F, t1992: F, t22635: F, t3886: F, t5353: F, t3888: F, t55118: F, t1799: F) -> (F, F, F, F, F) {
    let t90470 = t22751 * t26190;
    let t90472 = t6914 * t26356;
    let t90477 = t1992 * t22635 * t3886 * t5353 * t1385;
    let t90485 = t1992 * t22635 * t55118 * t3888;
    let t90488 = t3886 * t1799;
    (t90470, t90472, t90477, t90485, t90488)
}
