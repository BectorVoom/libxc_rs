//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 979/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk979<F: Float>(t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F, t21870: F, t1137: F) -> (F,) {
    let t21885 = 0.6311625e0 * t21781 + 0.3529725e1 * t21783 + 0.264729375e1 * t21786 - 0.20839e0 * t21789 + 0.62517e0 * t21792 + 0.104195e0 * t21795 + 0.57386111111111111112e0 * t21760 - 0.20659e1 * t21764 + 0.309885e1 * t21771 + 0.516475e0 * t21778 + 0.46308888888888888889e-1 * t21802 - 0.157790625e0 * t21804 - 0.103295e1 * t21767 + 0.309885e1 * t21774;
    let t21886 = t21870 + t21885;
    let t21887 = t21886 * t1137;
    (t21887,)
}
