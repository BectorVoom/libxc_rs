//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 502/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk502<F: Float>(t14077: F, t262: F, t2134: F, t78: F, t8: F, t271: F, t4765: F) -> (F, F, F, F, F) {
    let t14078 = t14077 * t262;
    let t14079 = t2134 * t14078;
    let t14081 = t78 * t8;
    let t14082 = F::new(1.0) / t14081;
    let t14083 = t14082 * t271;
    let t14084 = t4765 * t14083;
    (t14078, t14079, t14082, t14083, t14084)
}
