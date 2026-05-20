//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1871/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871<F: Float>(t252: F, t87230: F, t13230: F, t87052: F, t23168: F, t25321: F, t25284: F, t6579: F, t13388: F, t1888: F, t6646: F, t13385: F, t22996: F) -> (F, F, F, F, F) {
    let t87529 = t87230 * t252;
    let t87531 = t87052 * t87529 * t13230;
    let t87533 = t23168 * t25321;
    let t87535 = t6579 * t25284;
    let t87538 = t1888 * t6646 * t13388;
    let t87541 = t1888 * t22996 * t13385;
    (t87531, t87533, t87535, t87538, t87541)
}
