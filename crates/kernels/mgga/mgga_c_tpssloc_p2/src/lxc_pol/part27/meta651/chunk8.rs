//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2272/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2272<F: Float>(t45844: F, t6489: F, t12719: F, t72: F, t79: F, t1410: F, t9228: F, t2235: F, t3961: F, t3967: F, t1865: F, t22519: F, t22527: F, t22531: F, t22537: F, t22546: F, t26045: F, t26048: F, t26084: F, t6490: F, t6495: F, t7432: F, t7442: F, t83814: F) -> F {
    let t90330 = t45844 * t6489;
    let t90334 = t72 * t79 * t12719;
    let t90337 = t9228 * t1410;
    let t90340 = t2235 * t3961;
    let t90343 = t2235 * t3967;
    let t90346 = F::new(5.0) / F::new(3.0) * t26084 * t22527 + F::new(5.0) / F::new(6.0) * t26084 * t22531 + F::new(2.0) / F::new(3.0) * t22519 * t7442 + t22537 * t7442 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t6495 * t26045 + F::new(2.0) / F::new(3.0) * t6495 * t26048 - F::new(5.0) / F::new(3.0) * t83814 * t7432 - F::new(5.0) * t90330 * t22546 + F::new(5.0) / F::new(6.0) * t6490 * t90334 + t90337 * t1865 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t90340 * t1865 + F::new(2.0) / F::new(3.0) * t90343 * t1865;
    t90346
}
