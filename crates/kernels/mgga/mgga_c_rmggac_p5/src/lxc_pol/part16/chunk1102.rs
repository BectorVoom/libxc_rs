//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1102/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1102<F: Float>(t1743: F, t2228: F, t1614: F, t2471: F, t10350: F, t2265: F, t2604: F, t37860: F, t37866: F, t37872: F, t43481: F, t47110: F, t47112: F, t47114: F, t47119: F, t47133: F, t47138: F, t47142: F, t47146: F, t47152: F, t5879: F, t884: F) -> (F, F, F) {
    let t48894 = t2228 * t1743;
    let t48897 = t2471 * t1614;
    let t48901 = F::cast_from(0.11974241701863808564e0_f64) * t47110 - F::cast_from(0.1702583995731913576e-4_f64) * t47112 - F::cast_from(0.1702583995731913576e-4_f64) * t47114 - F::cast_from(0.1702583995731913576e-4_f64) * t47119 + t37860 - F::cast_from(0.11974241701863808564e0_f64) * t2604 * t10350 + t43481 - F::cast_from(0.5959043985061697516e-4_f64) * t47133 - F::cast_from(0.2363e1_f64) * t5879 * t2265 + F::cast_from(0.19863479950205658386e-4_f64) * t47138 - F::cast_from(0.2363e1_f64) * t37866 - F::cast_from(0.15323255961587222184e-3_f64) * t47142 - F::cast_from(0.15323255961587222184e-3_f64) * t47146 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t48894 + F::cast_from(0.11974241701863808564e0_f64) * t884 * t48897 - t37872 - F::cast_from(0.10215503974391481456e-3_f64) * t47152;
    (t48894, t48897, t48901)
}
