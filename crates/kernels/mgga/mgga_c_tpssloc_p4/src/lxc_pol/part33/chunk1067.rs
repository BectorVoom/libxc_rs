//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1067/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1067<F: Float>(t22430: F, t3: F, t1458: F, t5456: F, t5493: F, t1401: F, t16524: F, t20162: F, t20347: F, t3941: F, t5371: F, t576: F, t577: F) -> (F, F, F, F) {
    let t22431 = t3 * t22430;
    let t22445 = t5456 * t1458;
    let t22448 = t1458 * t5493;
    let t22453 = F::cast_from(0.45e1_f64) * t22430 * t577 + F::cast_from(0.405e2_f64) * t20162 * t1458 + F::cast_from(81.0_f64) * t16524 * t5456 + F::cast_from(0.405e2_f64) * t5371 * t5493 + F::cast_from(27.0_f64) * t576 * t22445 + F::cast_from(81.0_f64) * t3941 * t22448 + F::cast_from(0.135e2_f64) * t1401 * t20347;
    (t22431, t22445, t22448, t22453)
}
