//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 960/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk960<F: Float>(t1983: F, t25985: F, t1307: F, t1845: F, t8643: F, t22574: F, t15868: F, t2019: F, t1774: F, t6534: F, t652: F, t2314: F, t7468: F, t25965: F, t25969: F, t25973: F, t25975: F, t25977: F, t25979: F, t25982: F, t4028: F, t4034: F, t650: F, t6539: F, t7472: F, t7670: F) -> (F, F, F, F, F) {
    let t25987 = 3.0 * t1983 * t25985;
    let t25988 = t1845 * t1307;
    let t25989 = t8643 * t25988;
    let t25991 = 3.0 * t22574 * t25989;
    let t25992 = t2019 * t15868;
    let t25993 = t1983 * t25992;
    let t25994 = t1774 * t6534;
    let t25996 = 2.0 * t652 * t25994;
    let t25998 = 2.0 * t2314 * t7468;
    let t25999 = -2.0 * t25965 * t652 - 2.0 * t4028 * t6539 - 2.0 * t4034 * t7472 - t650 * t7670 - t25969 - t25973 - t25975 - t25977 - t25979 - t25982 + t25987 - t25991 - t25993 - t25996 - t25998;
    (t25988, t25989, t25992, t25994, t25999)
}
