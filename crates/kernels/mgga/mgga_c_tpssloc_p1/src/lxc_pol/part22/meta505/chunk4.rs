//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1955/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1955<F: Float>(t11247: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t1107: F) -> (F, F) {
    let t21780 = -t11247 + F::new(4.0) / F::new(9.0) * t14702 + F::new(2.0) / F::new(9.0) * t18203 - F::new(2.0) / F::new(3.0) * t18219 - t18229 / F::new(3.0) + F::new(10.0) / F::new(27.0) * t21760 - F::new(4.0) / F::new(3.0) * t21764 - F::new(2.0) / F::new(3.0) * t21767 + F::new(2.0) * t21771 + F::new(2.0) * t21774 + t21778 / F::new(3.0);
    let t21781 = t1107 * t21780;
    (t21780, t21781)
}
