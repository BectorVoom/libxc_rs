//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1065/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1065<F: Float>(t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F) -> F {
    let t21885 = F::cast_from(0.6311625e0_f64) * t21781 + F::cast_from(0.3529725e1_f64) * t21783 + F::cast_from(0.264729375e1_f64) * t21786 - F::cast_from(0.20839e0_f64) * t21789 + F::cast_from(0.62517e0_f64) * t21792 + F::cast_from(0.104195e0_f64) * t21795 + F::cast_from(0.57386111111111111112e0_f64) * t21760 - F::cast_from(0.20659e1_f64) * t21764 + F::cast_from(0.309885e1_f64) * t21771 + F::cast_from(0.516475e0_f64) * t21778 + F::cast_from(0.46308888888888888889e-1_f64) * t21802 - F::cast_from(0.157790625e0_f64) * t21804 - F::cast_from(0.103295e1_f64) * t21767 + F::cast_from(0.309885e1_f64) * t21774;
    t21885
}
