//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1707/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1707<F: Float>(t1985: F, t26351: F, t1842: F, t3886: F, t1385: F, t22635: F, t1992: F, t6883: F, t7697: F, t22897: F, t5336: F, t22751: F, t7733: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26352 = t1985 * t26351;
    let t26354 = t3886 * t1842;
    let t26355 = t26354 * t1385;
    let t26356 = t22635 * t26355;
    let t26357 = t1992 * t26356;
    let t26361 = t6883 * t7697;
    let t26378 = t22897 * t5336;
    let t26379 = t1992 * t26378;
    let t26381 = t22751 * t7733;
    (t26352, t26354, t26355, t26356, t26357, t26361, t26378, t26379, t26381)
}
