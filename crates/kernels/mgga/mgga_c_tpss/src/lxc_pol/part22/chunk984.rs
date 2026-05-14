//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 984/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk984<F: Float>(t11476: F, t11621: F, t3931: F, t11594: F, t11598: F, t11602: F, t11609: F, t11614: F, t11618: F, t2722: F, t2740: F, t3945: F, t8559: F, t8568: F, t8989: F, t9031: F, t9033: F, t9038: F, t967: F) -> (F,) {
    let t11622 = t11621 * t11476;
    let t11623 = t3931 * t11622;
    let t11628 = -t2740 * t11594 / 1152.0 + 5.0 / 6912.0 * t2740 * t11598 + t2740 * t11602 / 2304.0 - t8989 * t3945 / 432.0 + t2722 * t11609 / 1536.0 + t8559 * t11614 / 512.0 - t8568 * t11618 / 512.0 + t967 * t11623 / 768.0 + 19.0 / 2592.0 * t9031 + t9033 / 1296.0 + t9038;
    (t11628,)
}
