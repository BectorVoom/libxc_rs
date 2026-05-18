//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 861/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk861<F: Float>(t13798: F, t40138: F, t13802: F, t61965: F, t14131: F, t68422: F, t8421: F, t21714: F, t8426: F, t14125: F, t68622: F, t8416: F) -> (F, F, F, F, F) {
    let t75450 = t40138 * t13798;
    let t75452 = t61965 * t13802;
    let t75455 = t14131 * t68422 * t8421;
    let t75458 = t14131 * t21714 * t8426;
    let t75461 = t68622 * t14125 * t8416;
    (t75450, t75452, t75455, t75458, t75461)
}
