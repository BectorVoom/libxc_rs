//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1372/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1372<F: Float>(t1009: F, t343: F, t25490: F, t6746: F, t884: F, t10309: F, t10327: F, t11007: F, t11047: F, t11077: F, t1920: F, t1948: F, t1949: F, t23327: F, t23601: F, t23633: F, t23636: F, t23679: F, t23696: F, t345: F, t6687: F, t6785: F, t6786: F, t6797: F, t6799: F, t6800: F, t82513: F, t82605: F, t82618: F, t82620: F, t82625: F, t82629: F, t82633: F, t82635: F, t82637: F, t82638: F, t82643: F, t82653: F) -> F {
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    let t82657 = t82655 * t884 * t6746;
    let t82660 = -F::new(0.82246703342411321826e-2) * t82605 - F::new(0.82246703342411321825e-2) * t6687 * t10327 * t1949 + F::new(0.24674011002723396548e-1) * t6797 * t6799 * t11077 * t6800 + F::new(0.82246703342411321825e-2) * t1920 * t345 * t1948 * t11007 - F::new(0.16449340668482264365e-1) * t82618 - F::new(0.49348022005446793095e-1) * t23601 * t82620 * t23679 + F::new(0.16449340668482264365e-1) * t23633 * t82625 * t23636 + F::new(0.43864908449286038307e-1) * t82629 + F::new(0.54831135561607547884e-2) * t82633 - F::new(0.18277045187202515961e-2) * t82635 + F::new(0.82246703342411321825e-2) * t82513 * t82637 * t11047 * t82638 - F::new(0.82246703342411321826e-2) * t23327 * t82643 * t6786 - F::new(0.21932454224643019154e-1) * t6687 * t23696 * t6785 * t10309 - F::new(0.16449340668482264365e-1) * t82653 * t82657;
    t82660
}
