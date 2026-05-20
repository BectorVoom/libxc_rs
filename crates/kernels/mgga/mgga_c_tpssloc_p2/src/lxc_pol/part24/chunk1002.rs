//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1002/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1002<F: Float>(t11182: F, t11184: F, t11187: F, t11194: F, t11272: F, t11280: F, t1129: F, t11297: F, t11300: F, t11303: F, t11307: F, t11310: F, t11311: F, t11345: F, t11350: F, t11353: F, t11356: F, t11361: F, t1157: F, t3334: F, t3357: F, t3371: F, t3378: F, t3396: F, t3401: F, t3404: F) -> F {
    let t11364 = -t11182 - t11184 - t11187 + t11194 - t11272 - t11280 - F::cast_from(0.35089341735807877242e1_f64) * t11297 * t3378 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t11300 - F::new(6.0) * t11303 * t3334 + F::new(6.0) * t3357 * t11307 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t11311 + F::new(1.0) * t1129 * t11345 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t11353 + F::cast_from(0.17544670867903938621e1_f64) * t11356 * t1157 + F::cast_from(0.17544670867903938621e1_f64) * t3371 * t3396 + F::cast_from(0.51947577317044391276e2_f64) * t11361 * t3404;
    t11364
}
