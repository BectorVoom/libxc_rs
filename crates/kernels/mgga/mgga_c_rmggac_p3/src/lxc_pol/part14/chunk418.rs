//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 418/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk418<F: Float>(t151: F, t4103: F, t5: F, t1034: F, t421: F, t155: F, t1009: F, t422: F, t389: F, t1012: F, t1132: F, t381: F) -> (F, F, F, F, F, F, F) {
    let t4106 = F::cast_from(0.34450798614814814813e-2_f64) * t5 * t4103 * t151;
    let t4107 = t1034 * t421;
    let t4108 = t155 * t4107;
    let t4111 = F::new(60.0) * t1009 * t422;
    let t4114 = t1009 * t389;
    let t4116 = t1012 * t422;
    let t4118 = t1012 * t389;
    let t4120 = t381 * t1132;
    (t4106, t4108, t4111, t4114, t4116, t4118, t4120)
}
