//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1184/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1184<F: Float>(t111143: F, t1266: F, t12725: F, t12734: F, t15857: F, t2199: F, t2200: F, t2202: F, t2314: F, t26114: F, t26117: F, t26179: F, t30038: F, t30085: F, t30269: F, t30274: F, t30315: F, t30316: F, t30321: F, t30330: F, t3652: F, t4028: F, t4034: F, t45632: F, t510: F, t5113: F, t652: F, t8176: F, t8190: F, t8196: F, t8273: F, t8278: F, t8280: F, t90381: F, t91753: F) -> (F,) {
    let t111213 = -2.0 * t91753 * t2200 - 4.0 * t26179 * t8176 + 4.0 * t12734 * t8278 + 4.0 * t2314 * t30330 + 4.0 * t5113 * t30321 - 2.0 * t652 * t3652 * t8273 + 4.0 * t12734 * t8280 + 4.0 * t26114 * t8196 + 4.0 * t26117 * t8196 + 2.0 * t90381 * t2202 - 2.0 * t652 * t510 * t111143 + 2.0 * t45632 * t2202 - 4.0 * t2314 * t30316 - 4.0 * t4028 * t30038 - 4.0 * t12725 * t8190 + 4.0 * t2314 * t30269 - 4.0 * t4034 * t30274 - 2.0 * t652 * t15857 * t2199 - 2.0 * t4028 * t30085 - 4.0 * t652 * t1266 * t30315;
    (t111213,)
}
