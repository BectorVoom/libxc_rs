//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1064/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1064<F: Float>(t12156: F, t210: F, t214: F, t1307: F, t213: F, t221: F, t3719: F, t116: F, t547: F, t212: F, t2586: F, t12012: F) -> (F, F, F, F, F) {
    let t12217 = t210 * t214 * t12156;
    let t12220 = t213 * t1307;
    let t12222 = t221 * t12220 * t3719;
    let t12225 = t547 * t116;
    let t12226 = t212 * t1307;
    let t12227 = t12225 * t12226;
    let t12228 = t2586 * t12227;
    let t12231 = t210 * t214 * t12012;
    (t12217, t12222, t12225, t12228, t12231)
}
