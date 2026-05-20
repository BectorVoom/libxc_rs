//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2294/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294<F: Float>(t18375: F, t3536: F, t11697: F, t18968: F, t3577: F, t11539: F, t1174: F, t18232: F, t18215: F, t11665: F, t18371: F, t15569: F, t15572: F) -> (F, F, F, F, F, F) {
    let t66554 = t3536 * t18375;
    let t66566 = t3577 * t11697 * t18968;
    let t66571 = t1174 * t11539 * t18232;
    let t66575 = t1174 * t11539 * t18215;
    let t66597 = t11665 * t18371;
    let t66599 = t15569 * t15572;
    (t66554, t66566, t66571, t66575, t66597, t66599)
}
