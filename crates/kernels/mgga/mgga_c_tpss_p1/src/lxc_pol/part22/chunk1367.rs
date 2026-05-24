//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1367/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1367<F: Float>(t65639: F, t65643: F, t65647: F, t60725: F, t60731: F, t60733: F, t60739: F, t60744: F, t60750: F, t60752: F, t65636: F, t65641: F, t65645: F) -> F {
    let t67183 = F::new(7.0) / F::new(144.0) * t65639;
    let t67185 = F::new(7.0) / F::new(144.0) * t65643;
    let t67187 = F::new(119.0) / F::new(864.0) * t65647;
    let t67191 = -F::new(7.0) / F::new(24.0) * t60725 - F::new(35.0) / F::new(54.0) * t60731 + F::new(7.0) / F::new(72.0) * t60733 + t65636 / F::new(192.0) - F::new(7.0) / F::new(144.0) * t60739 - t67183 + t65641 / F::new(192.0) + t67185 - t65645 / F::new(192.0) - t67187 - F::new(35.0) / F::new(288.0) * t60744 - F::new(119.0) / F::new(432.0) * t60750 + F::new(7.0) / F::new(288.0) * t60752;
    t67191
}
