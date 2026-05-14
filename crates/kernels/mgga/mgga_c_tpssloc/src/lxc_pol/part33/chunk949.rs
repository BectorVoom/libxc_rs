//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 949/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk949<F: Float>(t11247: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t1107: F, t1100: F, t1661: F, t5992: F, t11265: F) -> (F, F, F, F) {
    let t21780 = -t11247 + 4.0 / 9.0 * t14702 + 2.0 / 9.0 * t18203 - 2.0 / 3.0 * t18219 - t18229 / 3.0 + 10.0 / 27.0 * t21760 - 4.0 / 3.0 * t21764 - 2.0 / 3.0 * t21767 + 2.0 * t21771 + 2.0 * t21774 + t21778 / 3.0;
    let t21781 = t1107 * t21780;
    let t21783 = t1100 * t21780;
    let t21785 = t5992 * t1661;
    let t21786 = t11265 * t21785;
    (t21781, t21783, t21785, t21786)
}
