//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 909/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk909<F: Float>(t33363: F, t7688: F, t28017: F, t89: F, t2040: F, t33214: F, t7796: F, t28030: F, t8533: F, t33231: F, t7458: F, t1873: F, t5449: F, t127553: F, t22574: F, t24432: F) -> (F, F, F, F, F, F, F, F) {
    let t128502 = 6.0 * t33363 * t7688;
    let t128507 = t89 * t28017;
    let t128509 = 2.0 * t128507 * t2040;
    let t128511 = 4.0 * t33214 * t7796;
    let t128513 = 2.0 * t28030 * t8533;
    let t128516 = 4.0 * t7458 * t33231;
    let t128521 = t5449 * t1873;
    let t128523 = 2.0 * t128521 * t2040;
    let t128535 = 6.0 * t22574 * t24432 * t127553;
    (t128502, t128509, t128511, t128513, t128516, t128521, t128523, t128535)
}
