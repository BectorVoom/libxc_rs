//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2685/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2685<F: Float>(t2221: F, t6328: F, t2223: F, t2225: F, t39571: F, t17: F, t2516: F, t6320: F, t19572: F, t750: F, t184: F, t56349: F) -> (F, F, F, F, F, F, F) {
    let t56390 = t2221 * t6328;
    let t56391 = F::cast_from(12.0_f64) * t56390;
    let t56392 = t2223 * t6328;
    let t56393 = F::cast_from(32.0_f64) * t56392;
    let t56394 = t2225 * t6328;
    let t56395 = F::cast_from(20.0_f64) * t56394;
    let t56396 = F::cast_from(96.0_f64) * t39571;
    let t56398 = t17 * t6320 * t2516;
    let t56400 = t17 * t19572 * t750;
    let t56401 = F::cast_from(2.0_f64) * t56400;
    let t56403 = t17 * t56349 * t184;
    (t56391, t56393, t56395, t56396, t56398, t56401, t56403)
}
