//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1364/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1364<F: Float>(t1279: F, t20128: F, t117: F, t547: F, t65458: F, t1281: F, t13279: F, t13283: F, t13286: F, t1670: F, t1784: F, t18575: F, t20094: F, t3410: F, t4556: F, t4559: F, t5766: F, t6284: F, t66101: F, t66111: F, t66114: F, t66121: F, t66123: F, t66125: F, t66127: F, t66129: F) -> (F,) {
    let t66131 = 6.0 * t1279 * t20128;
    let t66134 = 3.0 * t547 * t117 * t65458;
    let t66141 = 6.0 * t1281 * t20094 + 6.0 * t13279 * t1784 + 12.0 * t13283 * t1784 + 6.0 * t13286 * t1784 + 3.0 * t1670 * t18575 + 3.0 * t3410 * t6284 + 12.0 * t4556 * t5766 + 6.0 * t4559 * t5766 + t66101 + t66111 + t66114 + t66121 + t66123 + t66125 + t66127 + t66129 + t66131 + t66134;
    (t66141,)
}
