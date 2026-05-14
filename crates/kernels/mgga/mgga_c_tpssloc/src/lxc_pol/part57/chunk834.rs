//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 834/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk834<F: Float>(t114790: F, t6562: F, t7488: F, t2752: F, t33465: F, t33273: F, t81159: F, t33250: F, t6914: F, t115352: F, t6897: F, t7700: F, t1377: F, t7936: F, t33310: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t121753 = t6562 * t114790 * t7488;
    let t121782 = t33465 * t2752;
    let t122102 = t81159 * t33273;
    let t122112 = t6914 * t33250;
    let t122121 = t6897 * t115352 * t7700;
    let t122124 = t1377 * t7936;
    let t122133 = t6883 * t33310;
    (t121753, t121782, t122102, t122112, t122121, t122124, t122133)
}
