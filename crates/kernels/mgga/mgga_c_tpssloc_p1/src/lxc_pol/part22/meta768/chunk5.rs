//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2607/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607<F: Float>(t15740: F, t18371: F, t1222: F, t22175: F, t1090: F, t11728: F, t15569: F, t18300: F, t18383: F, t18946: F, t22312: F, t3578: F, t45114: F, t4582: F, t66052: F, t66054: F, t66057: F, t66073: F, t66076: F, t66079: F, t66084: F, t66092: F) -> F {
    let t72727 = t15740 * t18371;
    let t72733 = t22175 * t1222;
    let t72735 = -F::new(3.0) / F::new(512.0) * t11728 * t4582 * t18300 * t18946 - t66052 / F::new(576.0) + F::new(5.0) / F::new(3456.0) * t66054 - t66057 / F::new(324.0) + t66073 / F::new(2304.0) - t66076 / F::new(1152.0) - t66079 / F::new(1152.0) + t15569 * t18383 / F::new(288.0) - t66084 / F::new(384.0) + t66092 / F::new(384.0) - t72727 / F::new(1152.0) + t45114 * t3578 * t22312 * t1090 / F::new(768.0) - F::new(209.0) / F::new(3888.0) * t72733;
    t72735
}
