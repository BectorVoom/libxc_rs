//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1350/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1350<F: Float>(t1281: F, t13279: F, t13283: F, t13286: F, t13289: F, t1670: F, t1904: F, t19292: F, t20997: F, t3407: F, t4556: F, t6067: F, t6552: F, t66101: F, t66111: F, t66114: F, t66121: F, t66123: F, t66125: F, t66127: F, t66129: F, t66131: F, t66134: F) -> (F,) {
    let t68769 = 6.0 * t1281 * t20997 + 6.0 * t13279 * t1904 + 12.0 * t13283 * t1904 + 6.0 * t13286 * t1904 + 3.0 * t13289 * t1904 + 3.0 * t1670 * t19292 + 6.0 * t3407 * t6552 + 12.0 * t4556 * t6067 + t66101 + t66111 + t66114 + t66121 + t66123 + t66125 + t66127 + t66129 + t66131 + t66134;
    (t68769,)
}
