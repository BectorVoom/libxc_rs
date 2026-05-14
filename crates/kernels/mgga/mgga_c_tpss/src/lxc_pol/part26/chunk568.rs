//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 568/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk568<F: Float>(t294: F, t891: F, t2464: F, t928: F, t359: F, t361: F, t651: F, t355: F, t958: F, t962: F, t917: F, t921: F, t215: F, t334: F, t671: F, t333: F) -> (F, F, F, F, F, F, F, F) {
    let t2629 = t294 * t891;
    let t2644 = t928 * t2464;
    let t2650 = t359 * t651 * t361;
    let t2652 = t355 * t2650 / 13824.0;
    let t2660 = t958 * t962;
    let t2665 = t917 * t921;
    let t2668 = t215 * t671 * t334;
    let t2670 = t333 * t2668 / 432.0;
    (t2629, t2644, t2650, t2652, t2660, t2665, t2668, t2670)
}
