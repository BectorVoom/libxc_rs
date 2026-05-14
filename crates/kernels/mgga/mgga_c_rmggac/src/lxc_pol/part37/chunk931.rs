//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 931/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk931<F: Float>(t70124: F, t70131: F, t75928: F, t75932: F, t75946: F, t75951: F, t75954: F, t75958: F, t75964: F, t75968: F, t78376: F, t78377: F, t78378: F, t78379: F, t78384: F, t78385: F, t78391: F) -> (F,) {
    let t80386 = t78376 - t78377 - t78378 + t78379 - 0.81756761766873046873e-6 * t70124 - t70131 + 0.58171619854173713844e-5 * t75928 - 0.17451485956252114153e-4 * t75932 + t78384 - t78385 - 0.10511583655740820312e-5 * t75946 - t75951 - 0.93188427318671584242e-2 * t75954 + 0.15531404553111930707e-1 * t75958 + 0.62125618212447722828e-2 * t75964 - t78391 + 0.72714524817717142305e-5 * t75968;
    (t80386,)
}
