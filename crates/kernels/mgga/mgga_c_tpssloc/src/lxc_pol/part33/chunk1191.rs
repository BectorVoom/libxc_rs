//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1191/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1191<F: Float>(t28593: F, t383: F, t1058: F, t1920: F, t23619: F, t25465: F, t25508: F, t28597: F, t28602: F, t28605: F, t28610: F, t28614: F, t28618: F, t28622: F, t28626: F, t28631: F, t3200: F, t353: F, t4669: F, t6687: F, t6797: F, t7620: F) -> (F, F) {
    let t28634 = t383 * t28593;
    let t28636 = -t3200 * t28597 + F::new(2.0) * t4669 * t7620 + F::new(2.0) * t1058 * t28602 - F::new(0.16449340668482264365e-1) * t6797 * t28605 - t23619 - F::new(0.54831135561607547884e-2) * t25465 + F::new(0.54831135561607547884e-2) * t6687 * t28610 + F::new(0.27415567780803773942e-2) * t6687 * t28614 - F::new(0.54831135561607547884e-2) * t6687 * t28618 + F::new(0.82246703342411321825e-2) * t6797 * t28622 + F::new(0.16449340668482264365e-1) * t6797 * t28626 + F::new(0.54831135561607547884e-2) * t25508 + F::new(0.82246703342411321825e-2) * t1920 * t28631 + t353 * t28634;
    (t28634, t28636)
}
