//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 716/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk716<F: Float>(t4580: F, t70: F, t1290: F, t1306: F, t2009: F, t4573: F, t4579: F, t48: F, t455: F, t53: F, t2016: F, t60: F, t1300: F, t1303: F, t2024: F, t44: F, t56: F, t61: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t4581 = t4580 * t70;
    let t4584 = t1290 * t1306;
    let t4589 = t2009 * t4573;
    let t4592 = t48 * t4579;
    let t4596 = 1.0 / t53 / t455;
    let t4597 = sigma2 * t4596;
    let t4602 = t2016 * t4573;
    let t4605 = t60 * t4579;
    let t4608 = 5.0 / 18.0 * t44 * t4589 + 5.0 / 6.0 * t44 * t4592 + 88.0 / 9.0 * t4597 * t61 + 40.0 / 9.0 * t1300 * t1303 + 5.0 / 18.0 * t56 * t4602 - 5.0 / 6.0 * t56 * t4605 - t2024;
    (t4581, t4584, t4589, t4592, t4597, t4608)
}
