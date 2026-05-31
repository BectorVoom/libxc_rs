//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 48/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk48<F: Float>(t12: F, t13: F, t138: F, t145: F) -> (F, F, F, F, F) {
    let t158 = F::cast_from(1.0_f64) + F::cast_from(0.5137e-1_f64) * t12;
    let t163 = F::cast_from(0.705945e1_f64) * t13 + F::cast_from(0.1549425e1_f64) * t12 + F::cast_from(0.420775e0_f64) * t138 + F::cast_from(0.1562925e0_f64) * t145;
    let t166 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t163;
    let t167 = F::ln(t166);
    let t171 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t12;
    (t158, t163, t166, t167, t171)
}
