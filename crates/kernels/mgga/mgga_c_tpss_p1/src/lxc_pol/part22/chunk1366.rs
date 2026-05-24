//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1366/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1366<F: Float>(t65616: F, t65624: F, t65628: F, t62390: F, t65604: F, t65608: F, t65611: F, t65614: F, t65618: F, t65620: F, t65622: F, t65626: F, t65630: F) -> F {
    let t67169 = F::new(35.0) / F::new(144.0) * t65616;
    let t67173 = F::new(119.0) / F::new(3456.0) * t65624;
    let t67175 = F::new(7.0) / F::new(576.0) * t65628;
    let t67177 = t65604 / F::new(96.0) - t65608 / F::new(128.0) + t65611 / F::new(4.0) + t65614 / F::new(8.0) - t67169 + F::new(5.0) / F::new(96.0) * t65618 + F::new(5.0) / F::new(192.0) * t65620 - t65622 / F::new(768.0) - t62390 - t67173 - F::new(5.0) / F::new(32.0) * t65626 + t67175 - t65630 / F::new(768.0);
    t67177
}
