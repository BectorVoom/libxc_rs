//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 869/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk869<F: Float>(t13819: F, t8466: F, t2010: F, t70512: F, t8465: F, t14363: F, t15235: F, t15318: F, t14267: F, t2339: F, t3056: F, t2323: F) -> (F, F, F, F, F, F) {
    let t75580 = t13819 * t8466;
    let t75583 = t2010 * t8465 * t70512;
    let t75585 = t14363 * t15235;
    let t75587 = t14363 * t15318;
    let t75590 = t3056 * t14267 * t2339;
    let t75593 = t3056 * t14267 * t2323;
    (t75580, t75583, t75585, t75587, t75590, t75593)
}
