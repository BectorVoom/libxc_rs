//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2331/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2331<F: Float>(t28817: F, t6876: F, t1983: F, t28826: F, t83859: F, t26149: F, t7685: F, t100828: F, t100833: F, t100835: F, t100838: F, t100840: F, t1458: F, t1459: F, t19461: F, t19534: F, t1976: F, t2314: F, t24980: F, t25958: F, t28855: F, t4026: F, t4034: F, t5107: F, t5457: F, t6468: F, t652: F, t6862: F, t6872: F, t7451: F, t7458: F, t7670: F, t90400: F) -> F {
    let t100854 = F::new(6.0) * t6876 * t28817;
    let t100861 = F::new(6.0) * t1983 * t83859 * t28826;
    let t100863 = F::new(2.0) * t7685 * t26149;
    let t100864 = -F::new(4.0) * t1458 * t25958 * t652 - F::new(2.0) * t19534 * t1976 * t652 - F::new(4.0) * t1459 * t90400 - F::new(2.0) * t19461 * t1976 - F::new(4.0) * t2314 * t28855 - F::new(4.0) * t24980 * t7458 - F::new(4.0) * t28855 * t4034 - F::new(2.0) * t4026 * t7670 - F::new(2.0) * t5107 * t7451 - F::new(2.0) * t5457 * t6862 + t6468 * t6872 + t100828 - t100833 - t100835 + t100838 - t100840 + t100854 + t100861 - t100863;
    t100864
}
