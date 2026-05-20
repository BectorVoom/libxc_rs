//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1099/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099<F: Float>(t1864: F, t2244: F, t2245: F, t2250: F, t2283: F, t2304: F, t31: F, t33: F, t39103: F, t39110: F, t39213: F, t607: F, t628: F, t642: F, t65: F, t6509: F, t67: F, t80: F, t9247: F, t9248: F, t9251: F, t9258: F, t9259: F, t9260: F) -> F {
    let t39217 = -t39103 * t65 * t80 / F::new(4.0) - t607 * t628 * t67 * t9248 - t9247 * t6509 * t2250 - t9247 * t1864 * t9258 / F::new(3.0) - t31 * t39110 * t65 * t80 / F::new(12.0) - t9259 * t628 * t80 / F::new(3.0) - t9260 * t642 / F::new(3.0) - t2244 * t2283 * t80 / F::new(2.0) - t9251 * t642 - t2245 * t2304 / F::new(2.0) + t33 * t39213 * t80 / F::new(24.0);
    t39217
}
