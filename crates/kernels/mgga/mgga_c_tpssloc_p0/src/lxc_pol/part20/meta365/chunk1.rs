//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1699/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1699<F: Float>(t12606: F, t55: F, t12677: F, t12681: F, t12684: F, t12687: F, t12695: F, t12699: F, t12702: F, t1414: F, t1420: F, t2262: F, t2275: F, t2278: F, t39: F, t3982: F, t3985: F, t51: F, t615: F, t9311: F) -> F {
    let t12705 = t55 * t12606;
    let t12708 = F::new(220.0) / F::new(27.0) * t2262 * t1414 - F::new(40.0) / F::new(27.0) * t615 * t3982 - F::new(40.0) / F::new(9.0) * t615 * t3985 - F::new(5.0) / F::new(108.0) * t39 * t12677 + F::new(5.0) / F::new(9.0) * t39 * t12681 + F::new(5.0) / F::new(18.0) * t39 * t12684 + F::new(5.0) / F::new(6.0) * t39 * t12687 - F::new(20.0) / F::new(27.0) * t1420 * t2275 + F::new(20.0) / F::new(9.0) * t1420 * t2278 + F::new(5.0) / F::new(108.0) * t51 * t12695 + F::new(5.0) / F::new(9.0) * t51 * t12699 + F::new(5.0) / F::new(18.0) * t51 * t12702 - F::new(5.0) / F::new(6.0) * t51 * t12705 + t9311;
    t12708
}
