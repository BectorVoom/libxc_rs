//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1320/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1320<F: Float>(t18363: F, t1859: F, t1861: F, t19235: F, t19342: F, t19349: F, t582: F, t63495: F, t63587: F, t63597: F, t63600: F, t6472: F, t6475: F, t65169: F, t65172: F, t65175: F, t65178: F, t65182: F, t65189: F, t65214: F, t7690: F) -> (F,) {
    let t67934 = t18363 * t6472 / 3.0 + t18363 * t6475 / 3.0 + t65214 * t1861 / 3.0 - 10.0 / 3.0 * t65189 * t19235 - 10.0 / 3.0 * t65169 * t19235 - 10.0 / 3.0 * t65172 * t19235 - 10.0 / 3.0 * t65175 * t19235 - 10.0 / 3.0 * t19349 * t63597 - 10.0 / 3.0 * t19349 * t63600 + 35.0 * t63495 * t65182 + 20.0 * t7690 * t582 * t1859 * t19342 + 10.0 * t65178 * t63587;
    (t67934,)
}
