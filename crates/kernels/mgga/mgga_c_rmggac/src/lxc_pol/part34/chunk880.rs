//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 880/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk880<F: Float>(t118: F, t305: F, t69417: F, t69419: F, t69424: F, t76355: F, t78083: F, t78084: F, t78087: F, t78091: F, t78094: F, t78098: F, t78099: F, t78101: F, t78103: F, t78104: F) -> (F,) {
    let t78107 = t78083 + 0.59871208509319042821e-1 * t305 * t78084 + 0.59871208509319042821e-1 * t305 * t78087 - t78091 + t78094 - 0.16566831523319392755e-1 * t69417 + 0.49700494569958178265e-1 * t69419 - 0.82834157616596963775e-1 * t69424 + t78098 + t78099 + t78101 + t76355 + t78103 - 0.39914139006212695214e-1 * t118 * t78104;
    (t78107,)
}
