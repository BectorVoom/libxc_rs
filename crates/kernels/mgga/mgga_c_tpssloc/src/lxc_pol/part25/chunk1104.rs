//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1104/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1104<F: Float>(t12379: F, t6945: F, t22765: F, t3853: F, t22633: F, t22732: F, t3856: F, t6976: F, t12241: F, t1992: F, t22897: F, t22704: F, t22898: F, t80798: F) -> (F, F, F, F, F) {
    let t81005 = t6945 * t12379;
    let t81007 = t22765 * t3853;
    let t81016 = t22633 * t6976 * t22732 * t3856;
    let t81019 = t1992 * t22897 * t12241;
    let t81022 = t22704 * t80798 * t22898;
    (t81005, t81007, t81016, t81019, t81022)
}
