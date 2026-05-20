//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2704/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704<F: Float>(t12680: F, t1420: F, t16558: F, t19368: F, t19390: F, t19391: F, t19394: F, t19398: F, t20217: F, t20234: F, t2267: F, t39: F, t39159: F, t3966: F, t3981: F, t3991: F, t45970: F, t45974: F, t51: F, t5398: F, t5416: F, t607: F, t68513: F) -> F {
    let t75461 = F::new(5.0) / F::new(36.0) * t45974 * t68513 - F::new(5.0) / F::new(36.0) * t45970 * t68513 - F::new(5.0) / F::new(36.0) * t39 * t19368 * t3966 + F::new(5.0) / F::new(162.0) * t39 * t39159 * t20234 * t607 + F::new(5.0) / F::new(6.0) * t39 * t12680 * t5398 + F::new(5.0) / F::new(6.0) * t39 * t3981 * t16558 + F::new(5.0) / F::new(18.0) * t39 * t2267 * t20217 * t607 + F::new(220.0) / F::new(27.0) * t5416 * t3991 - F::new(40.0) / F::new(9.0) * t1420 * t19394 - F::new(10.0) / F::new(27.0) * t1420 * t19391 - F::new(20.0) / F::new(9.0) * t1420 * t19398 + F::new(5.0) / F::new(36.0) * t51 * t19390 * t3966;
    t75461
}
