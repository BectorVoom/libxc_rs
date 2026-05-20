//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2028/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2028<F: Float>(t102587: F, t1336: F, t1825: F, t19654: F, t19732: F, t24116: F, t27075: F, t27086: F, t27098: F, t29343: F, t29349: F, t3777: F, t5234: F, t5250: F, t5334: F, t6415: F, t6420: F, t7208: F, t84595: F, t84597: F, t91018: F, t91043: F, t91045: F, t93607: F, t97179: F, t97200: F) -> F {
    let t102790 = -F::new(2.0) * t5234 * t27098 - t84595 + F::new(2.0) * t5334 * t102587 * t5250 - t91018 + t84597 - F::new(2.0) * t1336 * t93607 * t1825 - F::new(2.0) * t3777 * t29349 - t1336 * t24116 * t6415 - t3777 * t29343 - t1336 * t24116 * t6420 - F::cast_from(0.23029076935875170111e0_f64) * t97179 + F::new(4.0) * t19654 * t27075 - t91043 + t91045 - t1336 * t7208 * t19732 - F::new(2.0) * t5234 * t27086 - F::cast_from(0.38381794893125283518e-1_f64) * t97200;
    t102790
}
