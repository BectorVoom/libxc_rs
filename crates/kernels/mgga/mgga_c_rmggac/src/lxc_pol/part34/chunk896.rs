//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 896/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk896<F: Float>(t70104: F, t70106: F, t70108: F, t70110: F, t75921: F, t75936: F, t75943: F, t739: F, t78112: F, t70124: F, t70130: F, t75928: F, t75932: F, t75946: F, t75951: F, t75954: F, t75958: F, t75964: F) -> (F,) {
    let t78375 = 0.638468998399467591e-4 * t70104;
    let t78376 = 0.1276937996798935182e-3 * t70106;
    let t78377 = 0.1915406995198402773e-3 * t70108;
    let t78378 = 0.638468998399467591e-4 * t70110;
    let t78379 = 0.14967802127329760705e-1 * t75921;
    let t78384 = 0.23268647941669485538e-4 * t75936;
    let t78385 = 0.3192344991997337955e-4 * t75943;
    let t78390 = t739 * t78112;
    let t78391 = 0.39914139006212695213e-1 * t78390;
    let t78392 = -t78375 + t78376 - t78377 - t78378 + t78379 - 0.81756761766873046877e-6 * t70124 - 0.15372131649401827111e-4 * t70130 + 0.58171619854173713846e-5 * t75928 - 0.17451485956252114154e-4 * t75932 + t78384 - t78385 - 0.10511583655740820313e-5 * t75946 - t75951 - 0.93188427318671584245e-2 * t75954 + 0.15531404553111930708e-1 * t75958 + 0.6212561821244772283e-2 * t75964 - t78391;
    (t78392,)
}
