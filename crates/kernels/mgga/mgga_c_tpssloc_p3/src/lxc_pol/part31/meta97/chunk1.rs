//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 599/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk599<F: Float>(t2221: F, t587: F, t591: F, t14: F, t21: F) -> (F, F, F, F) {
    let t2222 = F::new(0.1122e2) * t2221;
    let t2223 = t587 * t591;
    let t2224 = F::new(16.0) * t2223;
    let t2225 = t14 * t21;
    (t2222, t2223, t2224, t2225)
}
