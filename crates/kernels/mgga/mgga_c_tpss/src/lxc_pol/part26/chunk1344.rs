//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1344/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1344<F: Float>(t19235: F, t19342: F, t19349: F, t19411: F, t19414: F, t19417: F, t20713: F, t20769: F, t21139: F, t5979: F, t6080: F, t6472: F, t65175: F, t65189: F, t67953: F, t68003: F, t68006: F, t68009: F, t69186: F) -> (F,) {
    let t73052 = t21139 * t5979 / 3.0 + 2.0 / 3.0 * t19411 * t6472 + 2.0 / 3.0 * t19414 * t6472 + 2.0 / 3.0 * t19417 * t6472 + 2.0 / 3.0 * t6080 * t20769 - 10.0 / 3.0 * t65175 * t20713 - 10.0 / 3.0 * t19349 * t68003 - 10.0 / 3.0 * t19349 * t68006 - 10.0 / 3.0 * t19349 * t68009 - 10.0 * t67953 * t19342 - 10.0 / 3.0 * t65189 * t20713 - 10.0 / 3.0 * t69186 * t19235;
    (t73052,)
}
