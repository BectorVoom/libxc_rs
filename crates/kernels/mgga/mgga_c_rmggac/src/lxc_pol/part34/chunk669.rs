//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 669/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk669<F: Float>(t3351: F, t3352: F, t41091: F, t515: F, t68422: F, t68440: F, t8667: F, t21714: F, t8830: F, t14251: F, t73692: F, t15376: F, t69568: F, t68399: F, t21709: F, t68448: F, t73727: F) -> (F, F, F, F, F, F, F) {
    let t73837 = t3351 * t3352 * t515 * t41091;
    let t73840 = t68440 * t68422 * t8667;
    let t73843 = t68440 * t21714 * t8830;
    let t73845 = t73692 * t14251;
    let t73847 = t69568 * t15376;
    let t73849 = 0.24829349937757072982e-4 * t68399;
    let t73851 = t68448 * t21709 * t73727;
    (t73837, t73840, t73843, t73845, t73847, t73849, t73851)
}
