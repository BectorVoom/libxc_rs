//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1362/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362<F: Float>(t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F) -> F {
    let t77071 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68444 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t68446 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t68448 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68494 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t68498 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t76610 + F::cast_from(8.0_f64) * t76614 - F::cast_from(12.0_f64) * t76618 + F::cast_from(2.0_f64) * t76622 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t76626 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t59688 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t59694;
    t77071
}
