//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1333/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1333<F: Float>(t13213: F, t13268: F, t13331: F, t13375: F, t218: F, t1509: F, t852: F, t829: F, t252: F, t4233: F, t4182: F, t2684: F, t4282: F) -> (F, F, F, F, F, F, F) {
    let t13377 = t13213 + t13268 + t13331 + t13375;
    let t13378 = t218 * t13377;
    let t13380 = t852 * t1509;
    let t13381 = t13380 * t829;
    let t13384 = t252 * t4233;
    let t13385 = t13384 * t4182;
    let t13388 = t4282 * t2684;
    (t13377, t13378, t13380, t13381, t13384, t13385, t13388)
}
