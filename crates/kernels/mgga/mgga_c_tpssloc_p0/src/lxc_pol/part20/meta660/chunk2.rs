//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2466/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2466<F: Float>(t3199: F, t49649: F, t10470: F, t11045: F, t381: F, t1003: F, t10359: F, t11037: F, t11043: F, t11051: F, t14574: F, t14586: F, t14595: F, t14640: F, t1610: F, t1632: F, t3200: F, t3201: F, t3202: F, t3204: F, t4615: F, t4684: F, t4689: F, t49599: F, t50509: F, t50540: F) -> F {
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50616 = -F::new(6.0) * t14586 * t3200 * t4684 - F::new(3.0) * t14595 * t3200 * t4684 - F::new(3.0) * t3200 * t3201 * t50540 + F::new(3.0) * t49599 * t50509 * t50610 + F::new(3.0) * t1003 * t14640 + t10359 * t1632 - F::new(6.0) * t11037 * t14574 + t11043 * t1610 + F::new(3.0) * t11051 * t4689 - F::new(3.0) * t3202 * t50592 + F::new(3.0) * t3204 * t4615;
    t50616
}
