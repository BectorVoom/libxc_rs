//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1158/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158<F: Float>(t10021: F, t1336: F, t1361: F, t1369: F, t119: F, t12286: F, t12293: F, t12297: F, t12361: F, t1315: F, t1343: F, t210: F, t3733: F, t3783: F, t39622: F, t39892: F, t40012: F, t40019: F, t40022: F, t40025: F, t40026: F, t40035: F, t40044: F, t40047: F, t40052: F, t40054: F, t820: F) -> F {
    let t40059 = t1336 * t1361 * t10021;
    let t40060 = t40059 * t1369;
    let t40062 = F::new(7.0) / F::new(36.0) * t40012 - t1315 * t210 * t119 * t39892 / F::new(48.0) + F::new(35.0) / F::new(12.0) * t40019 + F::new(7.0) / F::new(3.0) * t40022 + F::new(5.0) / F::new(4.0) * t40025 * t210 * t119 * t40026 + F::new(3.0) / F::new(16.0) * t3733 * t210 * t119 * t39622 - t40035 * t12293 / F::new(128.0) + t12286 * t12297 / F::new(128.0) + t40044 * t1343 * t820 * t40047 / F::new(128.0) + F::new(35.0) / F::new(96.0) * t40052 + F::new(7.0) / F::new(96.0) * t40054 - t3783 * t12361 / F::new(192.0) + F::new(595.0) / F::new(648.0) * t40060;
    t40062
}
