//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 933/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk933<F: Float>(t15478: F, t16043: F, t3351: F, t3352: F, t44187: F, t515: F, t44239: F, t15457: F, t1971: F, t2144: F, t44232: F, t44194: F) -> (F, F, F, F, F, F) {
    let t76647 = t16043 * t15478;
    let t76648 = F::new(0.12769379967989351819e-4) * t76647;
    let t76651 = t3351 * t3352 * t515 * t44187;
    let t76652 = F::new(0.12769379967989351819e-4) * t76651;
    let t76655 = t3351 * t3352 * t515 * t44239;
    let t76656 = F::new(0.12769379967989351819e-4) * t76655;
    let t76657 = t16043 * t15457;
    let t76658 = F::new(0.12769379967989351819e-4) * t76657;
    let t76661 = t3351 * t1971 * t2144 * t44232;
    let t76662 = F::new(0.12769379967989351819e-4) * t76661;
    let t76665 = t3351 * t1971 * t2144 * t44194;
    (t76648, t76652, t76656, t76658, t76662, t76665)
}
