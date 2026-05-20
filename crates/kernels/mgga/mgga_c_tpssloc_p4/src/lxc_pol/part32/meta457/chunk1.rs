//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1733/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1733<F: Float>(t23083: F, t6606: F, t1891: F, t22822: F, t133: F, t6601: F, t6590: F, t6604: F) -> (F, F, F, F) {
    let t23084 = t23083 * t6606;
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23096 = F::cast_from(0.52708876011794399171e-3_f64) * t23095;
    let t23097 = t6590 * t6604;
    (t23084, t23093, t23096, t23097)
}
