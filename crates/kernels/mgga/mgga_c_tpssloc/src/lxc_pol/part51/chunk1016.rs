//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1016/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1016<F: Float>(t2031: F, t26024: F, t7032: F, t7428: F, t26012: F, t7031: F, t7445: F, t1860: F, t2032: F, t22549: F, t23963: F, t23968: F, t23970: F, t23973: F, t23978: F, t23995: F, t23999: F, t26009: F, t26016: F, t26028: F, t6486: F, t7035: F, t7782: F) -> (F,) {
    let t26945 = t2031 * t26024;
    let t26948 = t7428 * t7032;
    let t26954 = t2031 * t26012;
    let t26959 = t7031 * t7445;
    let t26960 = t1860 * t26959;
    let t26964 = t26028 * t2032 / 3.0 + t7428 * t7035 / 3.0 + t6486 * t7782 / 3.0 + t1860 * t26945 / 3.0 - 8.0 / 9.0 * t26948 - 8.0 / 9.0 * t23978 + t23995 - 8.0 / 9.0 * t23999 + 10.0 * t23963 * t26009 + 10.0 / 3.0 * t22549 * t26954 + 10.0 / 3.0 * t26016 * t23970 - 8.0 / 9.0 * t26960 + 40.0 / 9.0 * t23968 + 16.0 / 9.0 * t23973;
    (t26964,)
}
