//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1025/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1025<F: Float>(t2018: F, t26161: F, t6324: F, t92169: F, t33363: F, t7688: F, t28017: F, t89: F, t2040: F, t33214: F, t7796: F, t28030: F, t8533: F) -> (F, F, F, F, F) {
    let t128498 = F::cast_from(6.0_f64) * t26161 * t92169 * t2018 * t6324;
    let t128502 = F::cast_from(6.0_f64) * t33363 * t7688;
    let t128507 = t89 * t28017;
    let t128509 = F::cast_from(2.0_f64) * t128507 * t2040;
    let t128511 = F::cast_from(4.0_f64) * t33214 * t7796;
    let t128513 = F::cast_from(2.0_f64) * t28030 * t8533;
    (t128498, t128502, t128509, t128511, t128513)
}
