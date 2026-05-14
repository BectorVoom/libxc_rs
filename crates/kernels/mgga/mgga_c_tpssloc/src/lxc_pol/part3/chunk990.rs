//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 990/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk990<F: Float>(t14395: F, t2793: F, t10702: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10832: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F) -> (F, F) {
    let t14396 = t14395 * t2793;
    let t14398 = 0.51726012919273400301e3 * t10702 * t14396;
    let t14409 = 0.2283111111111111111e-1 * t13566;
    let t14410 = 0.11415555555555555555e-1 * t13602;
    let t14419 = -t10832 - 0.1522074074074074074e-1 * t10556 + 0.38051851851851851851e-2 * t10558 - 0.11415555555555555555e-1 * t10560 + 0.57077777777777777777e-2 * t10562 - 0.76103703703703703702e-2 * t13598 + 0.76103703703703703701e-2 * t13563 - t14409 + t14410 - 0.19025925925925925925e-1 * t13569 + 0.68493333333333333331e-1 * t13572 - 0.2283111111111111111e-1 * t13575 - 0.11415555555555555555e-1 * t13578 - 0.10274e0 * t13581 + 0.68493333333333333332e-1 * t13584 + 0.34246666666666666666e-1 * t13587 - 0.17123333333333333333e-1 * t13613;
    (t14398, t14419)
}
