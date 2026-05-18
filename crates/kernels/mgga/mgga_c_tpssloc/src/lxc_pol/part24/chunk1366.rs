//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1366/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1366<F: Float>(t3032: F, t3131: F, t23614: F, t82431: F, t11023: F, t11030: F, t11047: F, t23346: F, t23601: F, t23603: F, t23604: F, t23606: F, t23613: F, t23670: F, t23674: F, t23680: F, t23685: F, t23693: F, t23696: F, t23697: F, t23698: F, t25429: F, t2771: F, t6687: F, t6797: F, t6799: F, t6800: F, t6802: F, t82513: F, t82515: F, t82516: F, t82527: F, t82534: F, t82539: F, t82541: F) -> F {
    let t82542 = t3032 * t3131;
    let t82555 = t82431 * t23614;
    let t82561 = -F::new(0.10966227112321509577e-1) * t25429 * t23613 * t23697 + F::new(0.49348022005446793095e-1) * t82513 * t82515 * t11047 * t82516 + F::new(0.24674011002723396548e-1) * t6797 * t6799 * t11030 * t6800 + F::new(0.24125699647107321069e0) * t82527 * t6802 - F::new(0.65797362673929057459e-1) * t23670 * t23674 - F::new(0.13159472534785811492e0) * t82534 * t23680 + F::new(0.65797362673929057459e-1) * t82534 * t23606 + F::new(0.16449340668482264365e-1) * t82539 - F::new(0.49348022005446793095e-1) * t82513 * t82541 * t11047 * t82542 - F::new(0.24674011002723396548e-1) * t23601 * t23603 * t11023 * t23604 - F::new(0.21932454224643019154e-1) * t23346 * t23693 - F::new(0.29243272299524025538e-1) * t23346 * t23698 - F::new(0.54831135561607547883e-2) * t82555 + F::new(0.10966227112321509577e-1) * t6687 * t23696 * t23685 * t2771;
    t82561
}
