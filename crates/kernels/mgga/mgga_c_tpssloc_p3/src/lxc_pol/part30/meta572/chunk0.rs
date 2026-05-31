//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1942/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1942<F: Float>(t28593: F, t383: F, t1058: F, t1920: F, t23619: F, t25465: F, t25508: F, t28597: F, t28602: F, t28605: F, t28610: F, t28614: F, t28618: F, t28622: F, t28626: F, t28631: F, t3200: F, t353: F, t4669: F, t6687: F, t6797: F, t7620: F) -> (F, F) {
    let t28634 = t383 * t28593;
    let t28636 = -t3200 * t28597 + F::cast_from(2.0_f64) * t4669 * t7620 + F::cast_from(2.0_f64) * t1058 * t28602 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t28605 - t23619 - F::cast_from(0.54831135561607547884e-2_f64) * t25465 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t28610 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t28614 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t28618 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t28622 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t28626 + F::cast_from(0.54831135561607547884e-2_f64) * t25508 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t28631 + t353 * t28634;
    (t28634, t28636)
}
