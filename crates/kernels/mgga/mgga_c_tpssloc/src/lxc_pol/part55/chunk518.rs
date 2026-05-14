//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 518/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk518<F: Float>(t228: F, t68: F, t1484: F, t845: F, t776: F, t4119: F, t824: F, t1504: F, t1506: F, t230: F, t4219: F, t822: F, t825: F, t232: F) -> (F, F) {
    let t4225 = t228 * t68;
    let t4226 = t845 * t1484;
    let t4227 = t4226 * t776;
    let t4230 = t824 * t4119;
    let t4233 = 3.0 * t1504 * t825 + 3.0 * t1506 * t822 + 3.0 * t228 * t4230 - t230 * t4219 - 12.0 * t4225 * t4227;
    let t4234 = t4233 * t232;
    (t4233, t4234)
}
