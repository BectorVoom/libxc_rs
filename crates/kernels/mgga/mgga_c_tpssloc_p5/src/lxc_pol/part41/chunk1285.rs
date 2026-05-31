//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1285/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1285<F: Float>(t3: F, t30581: F, t1458: F, t8273: F, t2199: F, t5493: F, t1401: F, t16524: F, t20162: F, t28893: F, t30112: F, t30363: F, t30534: F, t3941: F, t5371: F, t5456: F, t577: F, t8207: F, t8294: F) -> (F, F, F, F) {
    let t30582 = t3 * t30581;
    let t30608 = t8273 * t1458;
    let t30611 = t2199 * t5493;
    let t30616 = F::cast_from(0.45e1_f64) * t30581 * t577 + F::cast_from(27.0_f64) * t30363 * t1458 + F::cast_from(27.0_f64) * t30112 * t5456 + F::cast_from(0.135e2_f64) * t8207 * t5493 + F::cast_from(0.135e2_f64) * t20162 * t2199 + F::cast_from(54.0_f64) * t16524 * t8294 + F::cast_from(27.0_f64) * t5371 * t8273 + F::cast_from(27.0_f64) * t28893 * t2199 + F::cast_from(54.0_f64) * t3941 * t30608 + F::cast_from(27.0_f64) * t3941 * t30611 + F::cast_from(0.135e2_f64) * t1401 * t30534;
    (t30582, t30608, t30611, t30616)
}
