//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 740/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk740<F: Float>(t23110: F, t6648: F, t23185: F, t226: F, t23026: F, t23029: F, t23032: F, t23038: F, t23151: F, t23156: F, t23160: F, t23167: F, t23170: F, t23174: F, t23178: F, t23182: F) -> (F, F) {
    let t23186 = t23110 * t6648;
    let t23187 = t23185 * t23186;
    let t23189 = -F::new(0.82246703342411321824e-2) * t23026 - t23029 + t23032 + F::new(0.49348022005446793095e-1) * t23038 + t226 * t23151 - F::new(0.3289868133696452873e-1) * t23156 - F::new(0.16449340668482264365e-1) * t23160 + t23167 + t23170 - t23174 - F::new(0.16449340668482264365e-1) * t23178 - F::new(0.82246703342411321825e-2) * t23182 + F::new(0.82246703342411321824e-2) * t23187;
    (t23187, t23189)
}
