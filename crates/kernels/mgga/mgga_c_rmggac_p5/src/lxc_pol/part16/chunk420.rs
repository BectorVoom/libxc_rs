//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 420/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk420<F: Float>(t142: F, t265: F, t6: F, t4130: F, t4133: F, t4136: F, t4138: F, t4142: F, t4144: F, t4146: F, t410: F, t417: F) -> (F, F, F) {
    let t4149 = t142 * t6 * t265;
    let t4151 = -F::cast_from(0.34523333333333333333e1_f64) * t4130 + F::cast_from(0.23015555555555555556e1_f64) * t4133 - F::cast_from(0.26851481481481481482e1_f64) * t4136 - F::cast_from(0.93932222222222222223e0_f64) * t4138 + F::new(0.73355e-1) * t4142 - F::new(0.14671e0) * t4144 - F::cast_from(0.17116166666666666667e0_f64) * t4146 - F::cast_from(0.36793333333333333333e0_f64) * t4149;
    let t4153 = t410 * t4151 * t417;
    (t4149, t4151, t4153)
}
