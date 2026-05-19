//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1110/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1110<F: Float>(t15882: F, t333: F, t326: F, t69418: F, t69420: F, t69424: F, t72038: F, t78069: F, t78072: F, t78073: F, t78077: F, t78078: F, t78079: F, t78083: F, t78091: F, t78094: F) -> (F, F) {
    let t80478 = t15882 * t333;
    let t80482 = -t78069 + t78072 + t78073 + t78077 + t78078 - t78079 - t72038 + t78083 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t80478 - t78091 + t78094 - t69418 + t69420 - F::cast_from(0.8283415761659696377e-1_f64) * t69424;
    (t80478, t80482)
}
