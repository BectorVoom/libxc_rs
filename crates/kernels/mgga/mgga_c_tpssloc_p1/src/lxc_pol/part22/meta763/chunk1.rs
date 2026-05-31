//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2571/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2571<F: Float>(t43859: F, t44249: F, t44275: F, t51299: F, t51310: F, t71203: F, t71206: F, t71499: F, t71501: F, t71505: F, t71508: F, t71511: F) -> F {
    let t71978 = F::cast_from(0.309885e1_f64) * t71203 + F::cast_from(0.929655e1_f64) * t71206 + t44249 + F::cast_from(0.794188125e1_f64) * t71499 - F::cast_from(0.473371875e0_f64) * t71501 - F::cast_from(0.30872592592592592592e0_f64) * t43859 - F::cast_from(0.62517e0_f64) * t71505 + F::cast_from(0.187551e1_f64) * t71508 + F::cast_from(0.13892666666666666667e0_f64) * t71511 - t51299 + t51310 + t44275;
    t71978
}
