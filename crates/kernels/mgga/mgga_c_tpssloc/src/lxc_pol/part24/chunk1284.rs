//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1284/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1284<F: Float>(t22886: F, t22892: F, t22893: F, t22751: F, t22887: F, t22633: F, t22732: F, t22897: F, t3793: F, t12273: F, t1992: F, t6976: F) -> (F, F, F, F) {
    let t81216 = t22892 * t22893 * t22886;
    let t81218 = t22751 * t22887;
    let t81222 = t22633 * t22897 * t22732 * t3793;
    let t81225 = t1992 * t6976 * t12273;
    (t81216, t81218, t81222, t81225)
}
