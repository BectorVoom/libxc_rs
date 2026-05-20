//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2212/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212<F: Float>(t1933: F, t23479: F, t88365: F, t23562: F, t25637: F, t984: F, t1014: F, t82654: F, t1022: F, t14037: F, t1611: F, t23419: F, t23556: F, t25655: F, t25661: F, t363: F, t378: F, t6747: F, t6800: F, t7583: F, t82971: F, t82996: F, t83085: F, t88400: F, t88407: F, t88415: F, t88422: F, t88425: F) -> F {
    let t88428 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88365 * t23479;
    let t88430 = t23562 * t25637 * t984;
    let t88431 = t82654 * t1014;
    let t88437 = -F::cast_from(0.32298204875312312684e-2_f64) * t88400 * t25655 + F::cast_from(0.16149102437656156342e-2_f64) * t88400 * t25661 - F::cast_from(0.20186378047070195428e-3_f64) * t88407 * t6747 - F::cast_from(0.20186378047070195428e-3_f64) * t82971 + F::new(19.0) / F::new(864.0) * t1611 * t23556 * t378 - t88415 - F::cast_from(0.10093189023535097714e-3_f64) * t83085 * t7583 + F::cast_from(0.10093189023535097714e-3_f64) * t82996 + F::new(5.0) / F::new(6912.0) * t23419 * t14037 - t88422 - t88425 - t88428 - F::cast_from(0.20186378047070195428e-3_f64) * t88430 * t88431 * t363 * t1022 * t6800;
    t88437
}
