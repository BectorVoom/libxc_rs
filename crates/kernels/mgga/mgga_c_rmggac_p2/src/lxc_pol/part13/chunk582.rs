//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 582/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk582<F: Float>(t118: F, t7694: F, t1986: F, t1994: F, t4685: F, t681: F, t2064: F, t321: F, t1550: F, t645: F, t839: F, t4044: F) -> (F, F, F, F, F, F, F) {
    let t7695 = t118 * t7694;
    let t7696 = t1986 * t7695;
    let t7697 = t1994 * t7696;
    let t7701 = t4685 * t681;
    let t7707 = t2064 * t321;
    let t7708 = t1550 * t7707;
    let t7710 = t645 * t839;
    let t7711 = t4044 * t7710;
    (t7696, t7697, t7701, t7707, t7708, t7710, t7711)
}
