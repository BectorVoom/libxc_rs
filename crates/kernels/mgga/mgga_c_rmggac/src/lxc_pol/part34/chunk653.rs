//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 653/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk653<F: Float>(t71564: F, t2019: F, t3180: F, t7926: F, t14572: F, t7487: F, t69995: F, t14559: F, t2020: F, t14696: F, t7345: F, t14588: F, t504: F, t14547: F, t275: F, t70018: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71565 = 0.15243824895787514157e-3 * t71564;
    let t71581 = t2019 * t7926 * t3180;
    let t71582 = 0.81300399444200075504e-3 * t71581;
    let t71583 = t7487 * t14572;
    let t71589 = 0.17451485956252114154e-3 * t69995;
    let t71594 = t2019 * t2020 * t14559;
    let t71606 = t7345 * t14696;
    let t71607 = 0.15243824895787514157e-3 * t71606;
    let t71608 = t504 * t14588;
    let t71615 = t275 * t14547;
    let t71619 = 0.50557909407869413937e0 * t70018;
    (t71565, t71582, t71583, t71589, t71594, t71607, t71608, t71615, t71619)
}
