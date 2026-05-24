//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 612/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk612<F: Float>(t201: F, t8607: F, t1979: F, t1982: F, t2320: F, t7691: F, t128: F, t1525: F, t118: F, t1986: F, t1994: F, t22: F, t7262: F) -> (F, F, F, F, F, F) {
    let t8608 = t8607 * t201;
    let t8610 = t8608 * t1979 * t1982;
    let t8612 = t7691 * t2320;
    let t8614 = t128 * t1525;
    let t8615 = t118 * t8614;
    let t8616 = t1986 * t8615;
    let t8617 = t1994 * t8616;
    let t8619 = t7262 * t22;
    (t8608, t8610, t8612, t8616, t8617, t8619)
}
