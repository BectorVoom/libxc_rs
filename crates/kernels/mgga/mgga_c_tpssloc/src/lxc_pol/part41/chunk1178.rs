//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1178/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1178<F: Float>(t1268: F, t12725: F, t19456: F, t2200: F, t26114: F, t26117: F, t28002: F, t28030: F, t30269: F, t30316: F, t30321: F, t30330: F, t30565: F, t4028: F, t5113: F, t5361: F, t6468: F, t7458: F, t7676: F, t8176: F, t8189: F, t8190: F, t8260: F, t8273: F, t8278: F, t96709: F, t97933: F) -> (F,) {
    let t112049 = 4.0 * t1268 * t5361 * t8273 + 2.0 * t1268 * t6468 * t8189 - 4.0 * t12725 * t8260 + 4.0 * t19456 * t8278 - 2.0 * t2200 * t96709 - 2.0 * t2200 * t97933 - 4.0 * t26114 * t8260 + 4.0 * t26114 * t8278 + 4.0 * t26117 * t8278 - 4.0 * t28002 * t8190 - 2.0 * t28030 * t8176 - 2.0 * t28030 * t8190 + 4.0 * t30269 * t4028 + 4.0 * t30269 * t7676 - 4.0 * t30316 * t4028 - 4.0 * t30316 * t7458 + 4.0 * t30321 * t7676 + 4.0 * t30330 * t4028 + 4.0 * t30330 * t7676 + 2.0 * t30565 * t5113;
    (t112049,)
}
