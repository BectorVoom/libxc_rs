//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 791/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk791<F: Float>(t15478: F, t16043: F, t3351: F, t3352: F, t44187: F, t515: F, t44239: F, t15457: F, t1971: F, t2144: F, t44232: F, t44194: F, t16156: F, t15504: F, t73791: F, t73797: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76647 = t16043 * t15478;
    let t76648 = 0.12769379967989351819e-4 * t76647;
    let t76651 = t3351 * t3352 * t515 * t44187;
    let t76652 = 0.12769379967989351819e-4 * t76651;
    let t76655 = t3351 * t3352 * t515 * t44239;
    let t76656 = 0.12769379967989351819e-4 * t76655;
    let t76657 = t16043 * t15457;
    let t76658 = 0.12769379967989351819e-4 * t76657;
    let t76661 = t3351 * t1971 * t2144 * t44232;
    let t76662 = 0.12769379967989351819e-4 * t76661;
    let t76665 = t3351 * t1971 * t2144 * t44194;
    let t76666 = 0.12769379967989351819e-4 * t76665;
    let t76667 = t16156 * t15478;
    let t76668 = 0.29795219925308487578e-4 * t76667;
    let t76669 = t16156 * t15504;
    let t76670 = 0.99317399751028291929e-5 * t76669;
    let t76671 = 0.19709219354514038085e-5 * t73791;
    let t76673 = 0.2627895913935205078e-5 * t73797;
    (t76648, t76652, t76656, t76658, t76662, t76666, t76668, t76670, t76671, t76673)
}
