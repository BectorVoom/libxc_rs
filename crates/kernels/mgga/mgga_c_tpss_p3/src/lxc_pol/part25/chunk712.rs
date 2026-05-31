//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 712/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk712<F: Float>(t2033: F, t4573: F, t4579: F, t608: F, t2040: F, t612: F, t77: F, t1291: F, t1307: F, t1314: F, t4574: F, t4581: F, t4584: F, t4609: F, t71: F, t85: F) -> (F, F, F) {
    let t4614 = t2033 * t4573;
    let t4616 = t608 * t4579;
    let t4618 = t2040 * t4573;
    let t4620 = t612 * t4579;
    let t4622 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4614 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4616 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4618 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4620;
    let t4623 = t77 * t4622;
    let t4626 = -t4574 * t85 / F::cast_from(12.0_f64) - t4581 * t85 / F::cast_from(12.0_f64) - t4584 * t85 / F::cast_from(6.0_f64) - t1291 * t1314 / F::cast_from(6.0_f64) + t4609 * t85 / F::cast_from(24.0_f64) + t1307 * t1314 / F::cast_from(12.0_f64) + t71 * t4623 / F::cast_from(24.0_f64);
    (t4622, t4623, t4626)
}
