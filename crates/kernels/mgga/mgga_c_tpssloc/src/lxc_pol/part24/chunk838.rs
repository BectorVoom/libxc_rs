//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 838/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk838<F: Float>(t10006: F, t10044: F, t2623: F, t2643: F, t2707: F, t4178: F, t831: F, t843: F, t9602: F, t9604: F, t9609: F, t9613: F, t9618: F, t9623: F, t9629: F, t9634: F, t9639: F, t9963: F) -> (F,) {
    let t10046 = -t2623 * t2707 / 256.0 - 119.0 / 1152.0 * t9602 + 7.0 / 384.0 * t9604 - 5.0 / 128.0 * t843 * t9609 - t9613 * t831 / 1024.0 + 5.0 / 256.0 * t843 * t9618 - t2643 * t9623 / 1024.0 - t4178 * t9629 / 128.0 + t4178 * t9634 / 512.0 - 7.0 / 192.0 * t9639 + t9963 + t10006 + t10044;
    (t10046,)
}
