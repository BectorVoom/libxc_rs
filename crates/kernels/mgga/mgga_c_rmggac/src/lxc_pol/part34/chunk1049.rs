//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1049/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1049<F: Float>(t78093: F, t69437: F, t69445: F, t25820: F, t77091: F, t27048: F, t77338: F, t14434: F, t1652: F, t118: F, t305: F, t69417: F, t69419: F, t69424: F, t76355: F, t78083: F, t78084: F, t78087: F, t78091: F) -> (F, F) {
    let t78094 = F::new(0.6818665413561335432e-1) * t78093;
    let t78098 = F::new(0.21819729323396273382e0) * t69437;
    let t78099 = F::new(0.54549323308490683456e-1) * t69445;
    let t78100 = t25820 * t77091;
    let t78101 = F::new(0.8980681276397856423e-1) * t78100;
    let t78103 = F::new(0.35922725105591425692e0) * t27048 * t77338;
    let t78104 = t14434 * t1652;
    let t78107 = t78083 + F::new(0.59871208509319042821e-1) * t305 * t78084 + F::new(0.59871208509319042821e-1) * t305 * t78087 - t78091 + t78094 - F::new(0.16566831523319392755e-1) * t69417 + F::new(0.49700494569958178265e-1) * t69419 - F::new(0.82834157616596963775e-1) * t69424 + t78098 + t78099 + t78101 + t76355 + t78103 - F::new(0.39914139006212695214e-1) * t118 * t78104;
    (t78104, t78107)
}
