//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 484/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk484<F: Float>(t305: F, t326: F, t3814: F, t3819: F, t3826: F, t3839: F, t3851: F, t4895: F, t4928: F, t5181: F, t5194: F, t5199: F, t5204: F, t5207: F, t5211: F, t5218: F, t5223: F, t5226: F, t5245: F, t551: F, t797: F, t851: F, t854: F, t861: F) -> F {
    let t5248 = -F::cast_from(0.11974241701863808564e0_f64) * t797 * t5211 + F::cast_from(0.26552308210121162678e-2_f64) * t851 * t5204 + F::cast_from(0.13276154105060581339e-2_f64) * t851 * t5207 - F::cast_from(0.59871208509319042821e-1_f64) * t797 * t5218 + F::cast_from(0.23948483403727617128e0_f64) * t3814 * t5181 - F::cast_from(0.148692925976678511e-1_f64) * t3819 * t5223 - F::cast_from(0.79656924630363488035e-2_f64) * t3826 * t5226 - F::cast_from(0.15931384926072697607e-2_f64) * t854 * t5218 + F::cast_from(0.3717323149416962775e-2_f64) * t861 * t5194 - F::cast_from(0.31862769852145395214e-2_f64) * t854 * t5211 - F::cast_from(0.15931384926072697607e-2_f64) * t854 * t5199 - F::cast_from(0.39914139006212695214e0_f64) * t3839 * t5223 - F::cast_from(0.11974241701863808564e0_f64) * t3851 * t5226 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t4928 + F::cast_from(0.19957069503106347607e-1_f64) * t305 * t4895 + F::cast_from(0.39914139006212695214e-1_f64) * t5245 * t551;
    t5248
}
