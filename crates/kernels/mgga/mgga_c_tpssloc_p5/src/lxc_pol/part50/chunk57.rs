//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 57/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk57<F: Float>(t123: F, t126: F, t129: F, t136: F) -> (F, F, F) {
    let t177 = F::cast_from(0.51785e1_f64) * t126 + F::cast_from(0.905775e0_f64) * t123 + F::cast_from(0.1100325e0_f64) * t129 + F::cast_from(0.1241775e0_f64) * t136;
    let t180 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t177;
    let t181 = F::ln(t180);
    (t177, t180, t181)
}
