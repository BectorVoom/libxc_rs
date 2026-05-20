//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2629/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2629<F: Float>(t18321: F, t4916: F, t1187: F, t15376: F, t18437: F, t18526: F, t3447: F, t3449: F, t4889: F, t4908: F, t4931: F, t52074: F, t52081: F, t52085: F, t64765: F, t64770: F, t64773: F, t64781: F, t64784: F, t64821: F, t71177: F, t73113: F, t73405: F, t73417: F, t73420: F, t73424: F, t73427: F) -> F {
    let t73433 = t18321 * t4916;
    let t73439 = -F::cast_from(0.44444444444444444443e-2_f64) * t15376 * t18437 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t3449 * t73405 - F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4908 * t71177 + F::cast_from(0.27777777777777777777e-3_f64) * t64765 + F::cast_from(0.55555555555555555554e-3_f64) * t64770 + F::cast_from(0.55555555555555555554e-3_f64) * t64773 + F::cast_from(0.27777777777777777777e-3_f64) * t64781 - F::cast_from(0.37037037037037037037e-3_f64) * t73417 + F::cast_from(0.27777777777777777777e-3_f64) * t73420 - F::cast_from(0.55555555555555555554e-3_f64) * t64784 - F::cast_from(0.37037037037037037036e-3_f64) * t64821 + F::cast_from(0.22222222222222222222e-2_f64) * t73424 - F::cast_from(0.27777777777777777777e-3_f64) * t73427 + F::cast_from(0.66666666666666666666e-2_f64) * t4889 * t18526 - F::cast_from(0.14814814814814814814e-2_f64) * t52074 - F::cast_from(0.9259259259259259259e-3_f64) * t52081 + t52085 - F::cast_from(0.8148148148148148148e-2_f64) * t73433 + F::cast_from(0.38024691358024691358e-1_f64) * t73113 * t1187 - F::cast_from(0.24444444444444444444e-1_f64) * t18321 * t4931;
    t73439
}
