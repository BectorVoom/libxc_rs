//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 905/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk905<F: Float>(t22724: F, t31594: F, t115332: F, t1985: F, t6907: F, t2085: F, t213: F, t225: F, t22633: F, t22637: F, t22642: F, t22643: F, t8621: F, t22635: F, t31558: F, t90506: F) -> (F, F, F, F, F) {
    let t115539 = t22724 * t31594;
    let t115540 = 0.26044789391763585244e-1 * t115539;
    let t115542 = t1985 * t115332 * t6907;
    let t115545 = t213 * t2085 * t225;
    let t115547 = t22633 * t115545 * t22637;
    let t115550 = t22642 * t22643 * t8621;
    let t115551 = 0.82246703342411321824e-2 * t115550;
    let t115554 = t22633 * t22635 * t31558 * t90506;
    (t115540, t115542, t115547, t115551, t115554)
}
