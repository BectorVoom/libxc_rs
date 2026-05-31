//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1019/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1019<F: Float>(t2018: F, t24432: F, t24995: F, t6330: F, t26161: F, t6324: F, t92169: F, t33363: F, t7688: F, t28017: F, t89: F, t2040: F) -> (F, F, F, F) {
    let t128492 = F::cast_from(6.0_f64) * t24995 * t24432 * t2018 * t6330;
    let t128498 = F::cast_from(6.0_f64) * t26161 * t92169 * t2018 * t6324;
    let t128502 = F::cast_from(6.0_f64) * t33363 * t7688;
    let t128507 = t89 * t28017;
    let t128509 = F::cast_from(2.0_f64) * t128507 * t2040;
    (t128492, t128498, t128502, t128509)
}
