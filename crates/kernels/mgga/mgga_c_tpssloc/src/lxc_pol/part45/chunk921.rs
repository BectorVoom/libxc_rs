//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 921/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk921<F: Float>(t115888: F, t31684: F, t31680: F, t9231: F, t131: F, t8511: F, t9239: F, t1862: F, t645: F, t113875: F, t641: F, t113876: F, t113864: F, t115833: F, t113871: F, t115863: F, t115866: F, t115871: F, t115873: F, t115877: F, t115880: F, t115884: F, t31672: F, t31677: F, t31681: F, t31693: F, t7026: F, t8512: F) -> (F,) {
    let t115889 = t115888 * t31684;
    let t115891 = t9231 * t31680;
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115896 = t1862 * t645;
    let t115898 = t113875 * t115896 * t641;
    let t115903 = t113875 * t1862;
    let t115904 = t115903 * t113876;
    let t115907 = t9239 * t31680;
    let t115908 = t115833 * t113864;
    let t115911 = -5.0 / 36.0 * t8512 * t115863 + 5.0 / 6.0 * t115866 * t31677 - 5.0 / 18.0 * t31672 * t31693 - 35.0 / 12.0 * t115871 * t115873 - 20.0 / 9.0 * t115877 + 5.0 / 18.0 * t7026 * t115880 + 5.0 / 18.0 * t31681 * t115884 - 40.0 / 27.0 * t115889 + 5.0 / 9.0 * t115891 * t31684 + 5.0 / 3.0 * t115895 * t115898 + 5.0 / 9.0 * t31681 * t113871 + 10.0 / 9.0 * t31681 * t115904 - 10.0 / 3.0 * t115907 * t115908;
    (t115911,)
}
