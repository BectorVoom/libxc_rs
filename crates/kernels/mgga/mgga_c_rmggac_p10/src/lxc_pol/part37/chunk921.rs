//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 921/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk921<F: Float>(t76665: F, t15478: F, t16156: F, t15504: F, t73791: F, t73797: F, t73799: F, t1971: F, t3351: F, t7262: F, t9571: F, t1986: F, t2467: F) -> (F, F, F, F, F, F, F, F) {
    let t76666 = F::cast_from(0.12769379967989351819e-4_f64) * t76665;
    let t76667 = t16156 * t15478;
    let t76668 = F::cast_from(0.29795219925308487578e-4_f64) * t76667;
    let t76669 = t16156 * t15504;
    let t76670 = F::cast_from(0.99317399751028291929e-5_f64) * t76669;
    let t76671 = F::cast_from(0.19709219354514038085e-5_f64) * t73791;
    let t76673 = F::cast_from(0.2627895913935205078e-5_f64) * t73797;
    let t76674 = F::cast_from(0.2627895913935205078e-5_f64) * t73799;
    let t76678 = t3351 * t1971 * t7262 * t9571;
    let t76679 = F::cast_from(0.25538759935978703639e-4_f64) * t76678;
    let t76680 = t1986 * t2467;
    (t76666, t76668, t76670, t76671, t76673, t76674, t76679, t76680)
}
