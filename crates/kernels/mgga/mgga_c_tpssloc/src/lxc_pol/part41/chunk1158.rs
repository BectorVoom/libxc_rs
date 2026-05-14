//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1158/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1158<F: Float>(t30315: F, t574: F, t1268: F, t12725: F, t19456: F, t2200: F, t2202: F, t2314: F, t26114: F, t26117: F, t26179: F, t4028: F, t5113: F, t7458: F, t7676: F, t8176: F, t8190: F, t8194: F, t8278: F, t8280: F) -> (F, F) {
    let t30330 = t30315 * t574;
    let t30347 = t1268 * t30330 - t12725 * t2200 - t19456 * t2200 + t19456 * t2202 - t2200 * t26114 - t2200 * t26179 + t2202 * t26117 + t2314 * t8278 + t2314 * t8280 - t4028 * t8176 - t4028 * t8190 + t4028 * t8194 + t5113 * t8278 + t5113 * t8280 - t7458 * t8176 + t7676 * t8194;
    (t30330, t30347)
}
