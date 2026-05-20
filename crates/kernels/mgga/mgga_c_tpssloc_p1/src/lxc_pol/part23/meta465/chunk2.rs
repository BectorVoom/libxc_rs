//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1362/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362<F: Float>(t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> F {
    let t77071 = F::new(4.0) / F::new(9.0) * t68444 + F::new(40.0) / F::new(81.0) * t68446 - F::new(16.0) / F::new(9.0) * t68448 + F::new(8.0) / F::new(9.0) * t68494 - F::new(8.0) / F::new(3.0) * t68498 - F::new(8.0) / F::new(9.0) * t76610 + F::new(8.0) * t76614 - F::new(12.0) * t76618 + F::new(2.0) * t76622 + F::new(8.0) / F::new(3.0) * t76626 + F::new(16.0) / F::new(9.0) * t59688 - F::new(8.0) / F::new(9.0) * t59694;
    t77071
}
