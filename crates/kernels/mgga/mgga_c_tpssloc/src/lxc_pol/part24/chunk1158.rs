//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1158/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1158<F: Float>(t80803: F, t80874: F, t80942: F, t81009: F, t22633: F, t22732: F, t3856: F, t6976: F, t12241: F, t1992: F, t22897: F, t22704: F, t22898: F, t80798: F, t12248: F, t6604: F) -> (F, F, F, F, F) {
    let t81011 = t80803 + t80874 + t80942 + t81009;
    let t81016 = t22633 * t6976 * t22732 * t3856;
    let t81019 = t1992 * t22897 * t12241;
    let t81022 = t22704 * t80798 * t22898;
    let t81027 = t6604 * t12248;
    (t81011, t81016, t81019, t81022, t81027)
}
