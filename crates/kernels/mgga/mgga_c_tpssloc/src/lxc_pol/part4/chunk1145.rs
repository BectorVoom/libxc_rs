//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1145/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1145<F: Float>(t11195: F, t14720: F, t14766: F, t14886: F, t14890: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18229: F, t18234: F, t18243: F, t18494: F, t18505: F, t18512: F, t18521: F, t18731: F, t18762: F, t18810: F, t18832: F) -> F {
    let t18834 = F::new(0.1898925e1) * t18731 - t11195 - F::new(0.54771111111111111111e-1) * t18512 + F::new(0.82156666666666666667e-1) * t18521 + F::new(0.66437037037037037037e-1) * t18203 - F::new(0.19931111111111111111e0) * t18219 - F::new(0.99655555555555555557e-1) * t18229 + F::new(0.29896666666666666667e0) * t18243 + F::new(0.18257037037037037037e-1) * t18494 - F::new(0.10954222222222222222e0) * t18505 + t18810 - F::new(0.9494625e0) * t18762 + F::new(0.18257037037037037037e0) * t14766 + F::new(0.13287407407407407407e0) * t14720 - t14886 - t14890 - F::new(0.19931111111111111111e0) * t18234 + F::new(0.33218518518518518518e0) * t18208 - F::new(0.11958666666666666667e1) * t18213 - F::new(0.39862222222222222222e0) * t18217 + F::new(0.17938e1) * t18223 + t18832;
    t18834
}
