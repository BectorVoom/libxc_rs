//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 419/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk419<F: Float>(t1344: F, t1353: F, t1356: F, t219: F, t654: F, t679: F, t726: F, t734: F, t739: F, t1364: F, t778: F, t222: F, t224: F) -> (F, F, F) {
    let t1373 = (t654 + t679 + t1344 + t1353 + t726 + t1356 - t734 - t739) * t219;
    let t1375 = t778 * t1364;
    let t1378 = -t1373 * t224 + 3.0 * t1375 * t222;
    (t1373, t1375, t1378)
}
