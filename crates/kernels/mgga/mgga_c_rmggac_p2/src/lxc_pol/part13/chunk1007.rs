//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1007/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1007<F: Float>(t25918: F, t8548: F, t4044: F, t5184: F, t645: F, t1632: F, t3352: F, t495: F, t511: F, t7230: F, t27075: F, t739: F, t7577: F) -> (F, F, F, F) {
    let t42068 = t25918 * t8548;
    let t42071 = t4044 * t645 * t5184;
    let t42076 = t7230 * t3352 * t511 * t1632 * t495;
    let t42081 = t739 * t7577 * t27075;
    (t42068, t42071, t42076, t42081)
}
