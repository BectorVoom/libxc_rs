//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 871/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk871<F: Float>(t4028: F, t6535: F, t19577: F, t8643: F, t22574: F, t7458: F, t2314: F, t7461: F, t4034: F, t1873: F, t5107: F, t652: F, t22591: F, t7687: F, t1983: F, t1307: F, t1845: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25969 = 2.0 * t4028 * t6535;
    let t25971 = t8643 * t19577;
    let t25973 = 3.0 * t22574 * t25971;
    let t25975 = 2.0 * t7458 * t6535;
    let t25977 = 2.0 * t2314 * t7461;
    let t25979 = 2.0 * t4034 * t7461;
    let t25980 = t5107 * t1873;
    let t25982 = 2.0 * t652 * t25980;
    let t25985 = t22591 * t7687;
    let t25987 = 3.0 * t1983 * t25985;
    let t25988 = t1845 * t1307;
    (t25969, t25971, t25973, t25975, t25977, t25979, t25980, t25982, t25985, t25987, t25988)
}
