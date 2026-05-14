//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 421/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk421<F: Float>(t1344: F, t1353: F, t1356: F, t1364: F, t1398: F, t198: F, t207: F, t654: F, t679: F, t726: F, t734: F, t739: F, t740: F, t823: F) -> (F,) {
    let t1402 = t1398 * t198 * t207 * t823 + 3.0 * t1364 * t198 * t740 + t1344 + t1353 + t1356 + t654 + t679 + t726 - t734 - t739;
    (t1402,)
}
