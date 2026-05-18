//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 843/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk843<F: Float>(t739: F, t75141: F, t14225: F, t3352: F, t8436: F, t1986: F, t305: F, t8441: F, t69619: F, t8446: F, t15397: F, t495: F) -> (F, F, F, F, F) {
    let t75143 = F::new(0.2993560425465952141e-1) * t739 * t75141;
    let t75145 = t14225 * t3352 * t8436;
    let t75148 = t1986 * t305 * t8441;
    let t75149 = t69619 * t75148;
    let t75152 = t14225 * t3352 * t8446;
    let t75154 = t15397 * t495;
    (t75143, t75145, t75149, t75152, t75154)
}
