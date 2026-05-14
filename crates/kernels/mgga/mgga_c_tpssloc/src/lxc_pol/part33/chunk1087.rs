//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1087/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1087<F: Float>(t28613: F, t6784: F, t5681: F, t6785: F, t5936: F, t6800: F, t6799: F, t5932: F, t1948: F, t5914: F, t345: F, t28593: F, t383: F, t1058: F, t1920: F, t23619: F, t25465: F, t25508: F, t28597: F, t28602: F, t28605: F, t28610: F, t3200: F, t353: F, t4669: F, t6687: F, t6797: F, t7620: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28614 = t6784 * t28613;
    let t28617 = t6785 * t5681;
    let t28618 = t6784 * t28617;
    let t28621 = t5936 * t6800;
    let t28622 = t6799 * t28621;
    let t28625 = t5932 * t6800;
    let t28626 = t6799 * t28625;
    let t28630 = t1948 * t5914;
    let t28631 = t345 * t28630;
    let t28634 = t383 * t28593;
    let t28636 = -t3200 * t28597 + 2.0 * t4669 * t7620 + 2.0 * t1058 * t28602 - 0.16449340668482264365e-1 * t6797 * t28605 - t23619 - 0.54831135561607547884e-2 * t25465 + 0.54831135561607547884e-2 * t6687 * t28610 + 0.27415567780803773942e-2 * t6687 * t28614 - 0.54831135561607547884e-2 * t6687 * t28618 + 0.82246703342411321825e-2 * t6797 * t28622 + 0.16449340668482264365e-1 * t6797 * t28626 + 0.54831135561607547884e-2 * t25508 + 0.82246703342411321825e-2 * t1920 * t28631 + t353 * t28634;
    (t28614, t28617, t28618, t28621, t28622, t28625, t28626, t28630, t28634, t28636)
}
