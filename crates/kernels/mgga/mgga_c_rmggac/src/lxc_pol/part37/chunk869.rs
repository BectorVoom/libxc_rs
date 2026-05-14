//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 869/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk869<F: Float>(t13951: F, t13956: F, t13960: F, t14535: F, t14536: F, t14538: F, t14539: F, t14540: F, t14541: F, t14542: F, t14550: F, t14945: F, t14946: F, t14947: F, t14950: F, t70679: F) -> (F,) {
    let t79947 = t14945 - t14535 - t14536 + t14946 + t14538 + t14539 - t14540 + t14541 + t14542 + t14947 - t14950 + t13951 - t13956 - t13960 + t14550 + t70679;
    (t79947,)
}
