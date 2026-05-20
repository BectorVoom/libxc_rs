//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1323/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1323<F: Float>(t110111: F, t110141: F, t110144: F, t110146: F, t110158: F, t110520: F, t110521: F, t110526: F, t110531: F, t110533: F, t110542: F, t1444: F, t2: F, t29907: F, t29911: F, t29922: F, t30175: F, t4049: F, t4067: F, t8128: F, t8137: F) -> F {
    let t110549 = -F::new(20.0) / F::new(9.0) * t110111 - F::new(5.0) / F::new(2.0) * t110520 * t110521 * t29911 + F::new(5.0) / F::new(9.0) * t110526 * t4049 * t29911 - t110531 + F::new(125.0) / F::new(72.0) * t110533 - F::new(25.0) / F::new(27.0) * t8137 * t110158 * t1444 + F::new(25.0) / F::new(36.0) * t30175 * t29922 * t2 - t110542 - F::new(5.0) / F::new(6.0) * t8128 * t29907 * t4067 + F::new(44.0) / F::new(9.0) * t110141 - F::new(110.0) / F::new(27.0) * t110144 - F::new(2.0) / F::new(3.0) * t110146;
    t110549
}
