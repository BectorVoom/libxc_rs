//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 971/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk971<F: Float>(t1441: F, t2593: F, t1429: F, t2549: F, t11207: F, t904: F, t1437: F, t2551: F, t10965: F, t10968: F, t10970: F, t10972: F, t11103: F, t11121: F, t11146: F, t11218: F, t2552: F, t2575: F, t2589: F, t2596: F, t3883: F, t896: F) -> (F,) {
    let t11362 = t1441 * t2593;
    let t11366 = t1429 * t2549;
    let t11371 = t11207 * t904;
    let t11374 = t1437 * t2551;
    let t11377 = -t10965 - t10968 - t10970 - t10972 - t11103 - 0.11696447245269292414e1 * t11362 * t2596 + t11146 - 0.19751673498613801407e-1 * t11121 + t11218 - 2.0 * t11366 * t2552 + 0.11696447245269292414e1 * t2589 * t3883 + 0.5848223622634646207e0 * t896 * t11371 + 6.0 * t2575 * t11374;
    (t11377,)
}
