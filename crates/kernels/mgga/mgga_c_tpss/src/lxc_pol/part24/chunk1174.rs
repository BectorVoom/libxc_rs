//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1174/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1174<F: Float>(t18289: F, t6245: F, t1760: F, t5706: F, t6246: F, t197: F, t507: F, t1759: F) -> (F, F, F, F, F) {
    let t19614 = t18289 * t6245;
    let t19616 = 3.0 * t1760 * t19614;
    let t19618 = 3.0 * t5706 * t6246;
    let t19619 = t197 * t507;
    let t19620 = t1759 * t19619;
    (t19614, t19616, t19618, t19619, t19620)
}
