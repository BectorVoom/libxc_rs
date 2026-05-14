//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 686/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk686<F: Float>(t3805: F, t3806: F, t3807: F, t1291: F, t2663: F, t1284: F, t67: F, t758: F, t2408: F, t2417: F, t2426: F, t2486: F, t3683: F, t3688: F, t3690: F, t3693: F, t3695: F) -> (F, F, F, F, F, F) {
    let t3809 = t3805 * t3806 * t3807;
    let t3813 = 0.24415263074675393405e-3 * t1291 * t2663;
    let t3814 = t1284 * t67;
    let t3815 = t3814 * t758;
    let t3816 = 0.36622894612013090108e-3 * t3815;
    let t3817 = t3813 - t2486 + t2408 + t2417 - t2426 - t3816 + t3688 + t3683 - t3690 - t3693 - t3695;
    (t3809, t3813, t3814, t3815, t3816, t3817)
}
