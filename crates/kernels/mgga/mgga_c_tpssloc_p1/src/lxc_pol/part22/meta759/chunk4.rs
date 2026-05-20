//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2553/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2553<F: Float>(t43859: F, t44027: F, t44053: F, t50919: F, t50948: F, t71203: F, t71206: F, t71499: F, t71501: F, t71505: F, t71508: F, t71511: F) -> F {
    let t71636 = F::new(0.17938e1) * t71203 + F::new(0.53814e1) * t71206 + t44027 + F::cast_from(0.427258125e1_f64) * t71499 - F::cast_from(0.230371875e0_f64) * t71501 - F::cast_from(0.2434271604938271605e0_f64) * t43859 - F::new(0.49294e0) * t71505 + F::new(0.147882e1) * t71508 + F::cast_from(0.10954222222222222222e0_f64) * t71511 - F::cast_from(0.26574814814814814815e0_f64) * t50919 + F::cast_from(0.79724444444444444446e0_f64) * t50948 + t44053;
    t71636
}
