//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 775/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk775<F: Float>(t607: F, t9681: F, t4194: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F, t2385: F, t686: F, t781: F, t685: F, t120: F, t118: F, t123: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9682 = t9681 * t607;
    let t9684 = 36.0 * t4194 * t9682;
    let t9688 = 1.0 / t126 / t136 * t116 / 4.0;
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    let t9692 = t2385 * t9691;
    let t9694 = t686 * t781;
    let t9695 = t685 * t9694;
    let t9697 = t120 * t781;
    let t9698 = t118 * t9697;
    let t9700 = 1.0/pow_3_2(t123);
    (t9684, t9689, t9691, t9692, t9694, t9695, t9697, t9698, t9700)
}
