//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 932/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk932<F: Float>(t8616: F, t9222: F, t118: F, t128: F, t1451: F, t1986: F, t8571: F, t10043: F, t674: F, t7715: F, t1997: F, t10084: F, t16043: F) -> (F, F, F, F) {
    let t45514 = t9222 * t8616;
    let t45519 = t8571 * t1986 * t118 * t128 * t1451;
    let t45522 = t10043 * t7715 * t674;
    let t45523 = t45522 * t1997;
    let t45525 = t16043 * t10084;
    (t45514, t45519, t45523, t45525)
}
