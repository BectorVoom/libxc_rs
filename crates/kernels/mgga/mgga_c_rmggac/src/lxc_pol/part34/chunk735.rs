//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 735/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk735<F: Float>(t68751: F, t68757: F, t68791: F, t68794: F, t68801: F, t68808: F, t14668: F, t16156: F, t14385: F, t34884: F, t14672: F, t68950: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t71031 = F::new(0.81700459932833791244e-6) * t68751;
    let t71033 = F::new(0.34547904762044099522e0) * t68757;
    let t71042 = F::new(0.86737941314158990616e-4) * t68791;
    let t71043 = F::new(0.162600798888400151e-2) * t68794;
    let t71046 = F::new(0.10492326631435615411e0) * t68801;
    let t71054 = F::new(0.26021382394247697184e-4) * t68808;
    let t71097 = t16156 * t14668;
    let t71109 = t34884 * t14385;
    let t71112 = t16156 * t14672;
    let t71151 = F::new(0.51300288795035171252e-6) * t68950;
    (t71031, t71033, t71042, t71043, t71046, t71054, t71097, t71109, t71112, t71151)
}
