//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 994/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk994<F: Float>(t13642: F, t13645: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t17398: F, t17420: F) -> (F,) {
    let t17422 = 0.142419375e1 * t17211 - 0.1898925e1 * t17213 - 0.9494625e0 * t17216 - 0.76790625e-1 * t17219 + 0.3071625e0 * t17221 + 0.15358125e0 * t17224 - 0.33218518518518518518e0 * t17154 + 0.11958666666666666667e1 * t17159 - 0.39862222222222222222e0 * t17163 - 0.17938e1 * t17169 + t17398 - 0.54771111111111111112e-1 * t17241 - 0.36514074074074074075e-1 * t17244 - 0.49293999999999999999e0 * t17247 + 0.32862666666666666666e0 * t17250 + 0.16431333333333333333e0 * t17253 - 0.27385555555555555556e-1 * t17256 + 0.1898925e1 * t17272 + 0.3071625e0 * t17274 - 0.18257037037037037037e0 * t13642 + t13645 + t17420;
    (t17422,)
}
