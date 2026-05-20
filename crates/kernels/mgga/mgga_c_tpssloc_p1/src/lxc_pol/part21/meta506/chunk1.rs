//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2149/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2149<F: Float>(t13598: F, t13650: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t13642: F, t13645: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t17398: F) -> F {
    let t17420 = -F::cast_from(0.26574814814814814815e0_f64) * t13598 + t13650 + F::cast_from(0.16431333333333333333e0_f64) * t17280 + F::cast_from(0.66437037037037037037e-1_f64) * t17149 - F::cast_from(0.19931111111111111111e0_f64) * t17165 + F::cast_from(0.99655555555555555557e-1_f64) * t17175 - F::cast_from(0.29896666666666666667e0_f64) * t17189 + F::cast_from(0.18257037037037037037e-1_f64) * t17286 - F::cast_from(0.10954222222222222222e0_f64) * t17288 + F::cast_from(0.54771111111111111111e-1_f64) * t17290 - F::cast_from(0.82156666666666666667e-1_f64) * t17293;
    let t17422 = F::cast_from(0.142419375e1_f64) * t17211 - F::new(0.1898925e1) * t17213 - F::new(0.9494625e0) * t17216 - F::new(0.76790625e-1) * t17219 + F::new(0.3071625e0) * t17221 + F::new(0.15358125e0) * t17224 - F::cast_from(0.33218518518518518518e0_f64) * t17154 + F::cast_from(0.11958666666666666667e1_f64) * t17159 - F::cast_from(0.39862222222222222222e0_f64) * t17163 - F::new(0.17938e1) * t17169 + t17398 - F::cast_from(0.54771111111111111112e-1_f64) * t17241 - F::cast_from(0.36514074074074074075e-1_f64) * t17244 - F::cast_from(0.49293999999999999999e0_f64) * t17247 + F::cast_from(0.32862666666666666666e0_f64) * t17250 + F::cast_from(0.16431333333333333333e0_f64) * t17253 - F::cast_from(0.27385555555555555556e-1_f64) * t17256 + F::new(0.1898925e1) * t17272 + F::new(0.3071625e0) * t17274 - F::cast_from(0.18257037037037037037e0_f64) * t13642 + t13645 + t17420;
    t17422
}
