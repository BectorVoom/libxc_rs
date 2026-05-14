//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 721/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk721<F: Float>(t1401: F, t2319: F, t2363: F, t3931: F, t3938: F, t3941: F, t577: F, t671: F, t2218: F, t2221: F, t2225: F, t2232: F, t1406: F, t604: F, t1437: F, t645: F) -> (F, F, F, F) {
    let t3946 = 0.45e1 * t3931 * t577 + 27.0 * t3938 * t671 + 27.0 * t3941 * t2319 + 0.135e2 * t1401 * t2363;
    let t3951 = -t2218 - 0.78e0 * t2221 - 0.578e2 * t2225 + t2232;
    let t3953 = t1406 * t604;
    let t3958 = t1437 * t645;
    (t3946, t3951, t3953, t3958)
}
