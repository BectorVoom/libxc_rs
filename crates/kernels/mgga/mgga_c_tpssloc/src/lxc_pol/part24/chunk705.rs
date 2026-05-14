//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 705/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk705<F: Float>(t6621: F, t849: F, t6580: F, t6582: F, t6587: F, t6594: F, t6603: F, t6607: F, t6610: F, t6615: F, t6618: F) -> (F,) {
    let t6622 = t6621 * t849;
    let t6624 = -t6580 - t6582 / 48.0 - t6587 - 0.12111826828242117256e-2 * t6594 - t6603 - 0.20186378047070195427e-3 * t6607 + t6610 / 1536.0 - t6615 / 1536.0 - t6618 - t6622 / 384.0;
    (t6624,)
}
