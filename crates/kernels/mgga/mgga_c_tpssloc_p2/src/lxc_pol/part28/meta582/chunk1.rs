//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1870/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1870<F: Float>(t22986: F, t25249: F, t2679: F, t6646: F, t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t25319: F, t2553: F, t6552: F, t6637: F) -> (F, F, F, F) {
    let t87517 = t22986 * t6646 * t25249 * t2679;
    let t87520 = t81651 * t23110 * t25299;
    let t87522 = t23168 * t25313;
    let t87527 = t6552 * t6637 * t25319 * t2553;
    (t87517, t87520, t87522, t87527)
}
