//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1048/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1048<F: Float>(t22886: F, t6637: F, t6888: F, t117: F, t547: F, t67: F, t6559: F) -> (F, F, F, F) {
    let t22887 = t6637 * t22886;
    let t22888 = t6888 * t22887;
    let t22891 = t547 * t67 * t117;
    let t22892 = t6559 * t22891;
    (t22887, t22888, t22891, t22892)
}
