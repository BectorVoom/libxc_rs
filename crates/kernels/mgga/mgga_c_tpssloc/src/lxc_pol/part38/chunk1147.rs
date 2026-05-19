//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1147/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1147<F: Float>(t10556: F, t10558: F, t10560: F, t10562: F, t10832: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t14409: F, t14410: F) -> F {
    let t14419 = -t10832 - F::cast_from(0.1522074074074074074e-1_f64) * t10556 + F::cast_from(0.38051851851851851851e-2_f64) * t10558 - F::cast_from(0.11415555555555555555e-1_f64) * t10560 + F::cast_from(0.57077777777777777777e-2_f64) * t10562 - F::cast_from(0.76103703703703703702e-2_f64) * t13598 + F::cast_from(0.76103703703703703701e-2_f64) * t13563 - t14409 + t14410 - F::cast_from(0.19025925925925925925e-1_f64) * t13569 + F::cast_from(0.68493333333333333331e-1_f64) * t13572 - F::cast_from(0.2283111111111111111e-1_f64) * t13575 - F::cast_from(0.11415555555555555555e-1_f64) * t13578 - F::new(0.10274e0) * t13581 + F::cast_from(0.68493333333333333332e-1_f64) * t13584 + F::cast_from(0.34246666666666666666e-1_f64) * t13587 - F::cast_from(0.17123333333333333333e-1_f64) * t13613;
    t14419
}
