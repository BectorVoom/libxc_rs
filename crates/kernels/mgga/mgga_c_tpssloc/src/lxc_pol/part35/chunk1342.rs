//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1342/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1342<F: Float>(t104280: F, t2132: F, t24746: F, t1210: F, t24721: F, t29593: F, t27700: F, t95422: F, t2136: F, t5398: F, t19040: F, t7345: F) -> (F, F, F, F, F) {
    let t104337 = t2132 * t104280 * t24746;
    let t104355 = t24721 * t1210 * t29593;
    let t104364 = t95422 * t27700;
    let t104367 = t2132 * t5398 * t2136;
    let t104369 = t7345 * t19040;
    (t104337, t104355, t104364, t104367, t104369)
}
