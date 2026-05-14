//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 928/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk928<F: Float>(t25988: F, t8643: F, t22574: F, t15868: F, t2019: F, t1983: F, t1774: F, t6534: F, t652: F, t2314: F, t7468: F, t4034: F, t1266: F, t7467: F, t6876: F, t7756: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25989 = t8643 * t25988;
    let t25991 = 3.0 * t22574 * t25989;
    let t25992 = t2019 * t15868;
    let t25993 = t1983 * t25992;
    let t25994 = t1774 * t6534;
    let t25996 = 2.0 * t652 * t25994;
    let t25998 = 2.0 * t2314 * t7468;
    let t26002 = 2.0 * t4034 * t7468;
    let t26003 = t1266 * t7467;
    let t26005 = 2.0 * t652 * t26003;
    let t26006 = t6876 * t7756;
    (t25989, t25991, t25992, t25993, t25994, t25996, t25998, t26002, t26003, t26005, t26006)
}
