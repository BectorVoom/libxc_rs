//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 818/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk818<F: Float>(t3787: F, t8617: F, t22724: F, t31594: F, t2085: F, t213: F, t225: F, t22642: F, t22643: F, t8621: F, t22716: F, t8612: F, t31569: F, t1862: F, t8308: F, t131: F, t63: F) -> (F, F, F, F, F, F, F, F) {
    let t115494 = t3787 * t8617;
    let t115539 = t22724 * t31594;
    let t115540 = 0.26044789391763585244e-1 * t115539;
    let t115545 = t213 * t2085 * t225;
    let t115550 = t22642 * t22643 * t8621;
    let t115551 = 0.82246703342411321824e-2 * t115550;
    let t115566 = t22716 * t8612;
    let t115567 = 0.63969658155208805863e-1 * t115566;
    let t115629 = t22724 * t31569;
    let t115630 = 0.26044789391763585244e-1 * t115629;
    let t115833 = t8308 * t1862;
    let t115834 = t63 * t131 * t115833;
    (t115494, t115540, t115545, t115551, t115567, t115630, t115833, t115834)
}
