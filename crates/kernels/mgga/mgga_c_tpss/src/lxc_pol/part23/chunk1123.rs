//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1123/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1123<F: Float>(t10445: F, t10456: F, t1165: F, t13133: F, t13136: F, t13146: F, t13220: F, t1338: F, t2056: F, t2105: F, t3493: F, t3537: F, t4347: F, t645: F, t7798: F, t4341: F) -> (F, F) {
    let t13223 = 4.0 * t10456 * t1338 + 2.0 * t1165 * t13220 + 4.0 * t13133 * t645 + 2.0 * t13146 * t1338 + 2.0 * t1338 * t7798 + 4.0 * t2056 * t3537 + 2.0 * t2105 * t3493 + 4.0 * t3537 * t4347 + t10445 + 2.0 * t13136;
    let t13225 = t4341 * t645;
    (t13223, t13225)
}
