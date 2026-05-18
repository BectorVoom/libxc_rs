//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1371/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1371<F: Float>(t2061: F, t6308: F, t42181: F, t5784: F, t10292: F, t18669: F, t5489: F, t6077: F, t62280: F, t18670: F, t19404: F, t19408: F) -> (F, F, F, F, F, F) {
    let t67316 = t6308 * t2061;
    let t67326 = t42181 * t5784;
    let t67329 = t10292 * t18669;
    let t67331 = F::new(80.0) / F::new(9.0) * t67329 * t5489;
    let t67333 = F::new(80.0) / F::new(9.0) * t62280 * t6077;
    let t67335 = F::new(80.0) / F::new(9.0) * t18670 * t19404;
    let t67337 = F::new(80.0) / F::new(9.0) * t18670 * t19408;
    (t67316, t67326, t67331, t67333, t67335, t67337)
}
