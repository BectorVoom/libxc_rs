//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 978/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk978<F: Float>(t1409: F, t9330: F, t2298: F, t3966: F, t12595: F, t12598: F, t12606: F, t2244: F, t2250: F, t4007: F, t4012: F, t607: F, t634: F, t638: F) -> F {
    let t12609 = t9330 * t1409;
    let t12612 = t2298 * t3966;
    let t12619 = -F::new(280.0) / F::new(27.0) * t12595 * t2244 + F::new(56.0) / F::new(9.0) * t12598 * t607 + F::new(28.0) / F::new(9.0) * t4007 * t2250 - F::new(4.0) / F::new(3.0) * t634 * t12606 + F::new(280.0) / F::new(27.0) * t12609 * t2244 + F::new(56.0) / F::new(9.0) * t12612 * t607 + F::new(28.0) / F::new(9.0) * t4012 * t2250 + F::new(4.0) / F::new(3.0) * t638 * t12606;
    t12619
}
