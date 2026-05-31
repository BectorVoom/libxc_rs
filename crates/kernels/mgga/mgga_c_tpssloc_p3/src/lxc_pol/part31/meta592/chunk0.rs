//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1837/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1837<F: Float>(t81375: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t22573: F, t7684: F, t27240: F, t580: F, t1395: F, t7961: F) -> (F, F, F, F, F, F) {
    let t91496 = F::cast_from(0.25587863262083522346e0_f64) * t81375;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91655 = t7684 * t22573;
    let t91830 = F::cast_from(2.0_f64) * t27240 * t580;
    let t91832 = F::cast_from(2.0_f64) * t1395 * t7961;
    (t91496, t91531, t91548, t91655, t91830, t91832)
}
