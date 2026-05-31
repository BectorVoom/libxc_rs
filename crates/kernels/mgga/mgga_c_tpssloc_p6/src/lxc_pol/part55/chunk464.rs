//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 464/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk464<F: Float>(t133: F, t2402: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t702: F, t683: F) -> (F, F) {
    let t2403 = t133 * t2402;
    let t2405 = -F::cast_from(0.42198333333333333333e0_f64) * t2388 + F::cast_from(0.84396666666666666666e0_f64) * t2391 + F::cast_from(0.39862222222222222223e0_f64) * t2394 + F::cast_from(0.68258333333333333333e-1_f64) * t2398 + F::cast_from(0.13651666666666666667e0_f64) * t2400 + F::cast_from(0.13692777777777777778e0_f64) * t2403;
    let t2406 = t2405 * t702;
    let t2408 = F::cast_from(1.0_f64) * t683 * t2406;
    (t2403, t2408)
}
