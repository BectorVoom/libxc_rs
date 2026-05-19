//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 556/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk556<F: Float>(t2398: F, t219: F, t807: F, t251: F, t810: F, t73: F, t818: F, param_beta: F) -> (F, F, F, F, F) {
    let t2399 = param_beta * t2398;
    let t2401 = t807 * t219;
    let t2405 = F::new(1.0) / t810 / t251;
    let t2406 = t73 * t2405;
    let t2407 = t818 * t818;
    (t2399, t2401, t2405, t2406, t2407)
}
