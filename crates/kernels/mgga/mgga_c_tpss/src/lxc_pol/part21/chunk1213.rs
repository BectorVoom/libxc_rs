//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1213/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1213<F: Float>(t1772: F, t18483: F, t18496: F, t19527: F, t19531: F, t19536: F, t19540: F, t19543: F, t19548: F, t19552: F, t19555: F, t19559: F, t19564: F, t19567: F, t5737: F, t5739: F, t6268: F, t6271: F) -> (F,) {
    let t19569 = -t1772 * t19567 + t18483 * t6268 - 2.0 * t18496 * t19536 + 2.0 * t19527 * t5739 + 2.0 * t19531 * t5739 - 2.0 * t19540 * t19543 + t19540 * t19555 + t19548 * t5739 + t19552 * t5739 + 2.0 * t19559 * t5739 + t19564 * t5739 - t5737 * t6271;
    (t19569,)
}
