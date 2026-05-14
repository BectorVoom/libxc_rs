//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 829/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk829<F: Float>(t32735: F, t6883: F, t32769: F, t33662: F, t576: F, t33334: F, t532: F, t1437: F, t1862: F, t115888: F, t33568: F, t12571: F, t31680: F, t115876: F, t33564: F, t31688: F, t33572: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120610 = t6883 * t32735;
    let t120632 = t6883 * t32769;
    let t120857 = t576 * t33662;
    let t120955 = t532 * t33334;
    let t121022 = t1862 * t1437;
    let t121029 = t115888 * t33568;
    let t121058 = t12571 * t31680;
    let t121064 = t115876 * t33564;
    let t121066 = t31688 * t33572;
    (t120610, t120632, t120857, t120955, t121022, t121029, t121058, t121064, t121066)
}
