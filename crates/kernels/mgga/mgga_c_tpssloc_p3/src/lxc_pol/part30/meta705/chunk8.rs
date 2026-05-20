//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2316/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2316<F: Float>(t23384: F, t28660: F, t28614: F, t362: F, t5914: F, t14618: F, t23327: F, t23670: F, t23685: F, t25568: F, t25708: F, t25713: F, t28605: F, t28631: F, t4669: F, t5685: F, t5903: F, t6680: F, t6687: F, t6784: F, t6813: F, t7603: F, t884: F, t89532: F, t89546: F, t99921: F) -> F {
    let t100431 = t23384 * t28660;
    let t100436 = t23384 * t28614;
    let t100449 = t362 * t5914;
    let t100459 = -F::cast_from(0.27415567780803773942e-2_f64) * t100431 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t99921 * t25713 + F::cast_from(0.91385225936012579807e-3_f64) * t100436 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t23685 * t5685 + t5903 * t6813 + F::new(4.0) * t14618 * t25708 - F::cast_from(0.21932454224643019153e-1_f64) * t6680 * t28631 + F::new(2.0) * t4669 * t25568 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t100449 * t884 + F::cast_from(0.43864908449286038307e-1_f64) * t23670 * t28605 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t89532 * t7603 + t89546;
    t100459
}
