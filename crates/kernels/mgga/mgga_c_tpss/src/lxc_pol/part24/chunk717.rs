//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 717/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk717<F: Float>(t38: F, t4608: F, t2033: F, t4573: F, t4579: F, t608: F, t2040: F, t612: F, t77: F, t1291: F, t1307: F, t1314: F, t4574: F, t4581: F, t4584: F, t71: F, t85: F) -> (F, F, F, F) {
    let t4609 = t38 * t4608;
    let t4614 = t2033 * t4573;
    let t4616 = t608 * t4579;
    let t4618 = t2040 * t4573;
    let t4620 = t612 * t4579;
    let t4622 = 28.0 / 9.0 * t4614 - 4.0 / 3.0 * t4616 + 28.0 / 9.0 * t4618 + 4.0 / 3.0 * t4620;
    let t4623 = t77 * t4622;
    let t4626 = -t4574 * t85 / 12.0 - t4581 * t85 / 12.0 - t4584 * t85 / 6.0 - t1291 * t1314 / 6.0 + t4609 * t85 / 24.0 + t1307 * t1314 / 12.0 + t71 * t4623 / 24.0;
    (t4609, t4622, t4623, t4626)
}
