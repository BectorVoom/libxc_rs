//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1145/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1145<F: Float>(t10556: F, t10558: F, t10560: F, t10562: F, t10832: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F, t14409: F, t14410: F) -> F {
    let t14419 = -t10832 - F::new(0.1522074074074074074e-1) * t10556 + F::new(0.38051851851851851851e-2) * t10558 - F::new(0.11415555555555555555e-1) * t10560 + F::new(0.57077777777777777777e-2) * t10562 - F::new(0.76103703703703703702e-2) * t13598 + F::new(0.76103703703703703701e-2) * t13563 - t14409 + t14410 - F::new(0.19025925925925925925e-1) * t13569 + F::new(0.68493333333333333331e-1) * t13572 - F::new(0.2283111111111111111e-1) * t13575 - F::new(0.11415555555555555555e-1) * t13578 - F::new(0.10274e0) * t13581 + F::new(0.68493333333333333332e-1) * t13584 + F::new(0.34246666666666666666e-1) * t13587 - F::new(0.17123333333333333333e-1) * t13613;
    t14419
}
