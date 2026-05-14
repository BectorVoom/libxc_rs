//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1109/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1109<F: Float>(t23661: F, t3201: F, t1058: F, t1920: F, t1950: F, t23323: F, t23327: F, t23601: F, t23606: F, t23610: F, t23614: F, t23619: F, t23621: F, t23626: F, t23629: F, t23633: F, t23637: F, t23642: F, t23644: F, t23647: F, t23650: F, t23654: F, t23658: F, t3180: F, t3200: F, t6687: F, t6797: F, t6811: F) -> (F, F) {
    let t23662 = t23661 * t3201;
    let t23664 = -0.82246703342411321825e-2 * t23601 * t23606 + 0.16449340668482264365e-1 * t6797 * t23610 - 0.54831135561607547884e-2 * t23327 * t23614 - t23619 + 0.82246703342411321825e-2 * t1920 * t23621 + 0.80418998823691070228e-1 * t23323 * t1950 - 0.14621636149762012769e-1 * t23626 + 0.54831135561607547884e-2 * t23629 + 0.54831135561607547884e-2 * t23633 * t23637 + 2.0 * t3180 * t6811 - 0.54831135561607547884e-2 * t23642 - 0.82246703342411321825e-2 * t6687 * t23644 - 0.16449340668482264365e-1 * t6687 * t23647 - 0.82246703342411321825e-2 * t6687 * t23650 + 2.0 * t1058 * t23654 - 0.16449340668482264365e-1 * t6797 * t23658 - t3200 * t23662;
    (t23662, t23664)
}
