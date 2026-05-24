//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 906/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk906<F: Float>(t27120: F, t739: F, t7577: F, t2001: F, t2281: F, t305: F, t321: F, t7720: F, t495: F, t511: F, t7230: F, t7231: F, t9104: F) -> (F, F, F) {
    let t40027 = t739 * t7577 * t27120;
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40032 = t7720 * t40031;
    let t40037 = t7230 * t7231 * t511 * t9104 * t495;
    (t40027, t40032, t40037)
}
