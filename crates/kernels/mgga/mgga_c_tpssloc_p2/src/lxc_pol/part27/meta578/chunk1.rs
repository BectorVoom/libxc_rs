//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2028/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2028<F: Float>(t22720: F, t6883: F, t22716: F, t6983: F, t22742: F, t6914: F, t22748: F, t80727: F, t22723: F, t268: F, t534: F, t22706: F) -> (F, F, F, F, F, F) {
    let t81037 = t6883 * t22720;
    let t81039 = t22716 * t6983;
    let t81041 = t6914 * t22742;
    let t81043 = t80727 * t22748;
    let t81046 = t22723 * t534 * t268;
    let t81047 = t81046 * t22706;
    (t81037, t81039, t81041, t81043, t81046, t81047)
}
