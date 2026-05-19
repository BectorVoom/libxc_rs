//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1056/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1056<F: Float>(t75423: F, t69313: F, t75419: F, t75421: F, t75425: F, t78165: F, t78166: F, t78167: F, t78168: F, t78169: F, t78170: F, t78171: F, t78172: F, t78173: F, t78174: F, t78175: F, t78176: F) -> F {
    let t78179 = F::cast_from(0.14464861606874801909e-3_f64) * t75423;
    let t78181 = F::cast_from(0.35403077613494883571e-2_f64) * t69313;
    let t78182 = -t78165 + t78166 + t78167 - t78168 + t78169 + t78170 - t78171 + t78172 - t78173 - t78174 - t78175 - t78176 - F::cast_from(0.45360193192290319574e-3_f64) * t75419 + F::cast_from(0.63504270469206447404e-3_f64) * t75421 + t78179 - F::cast_from(0.19286482142499735878e-3_f64) * t75425 - t78181;
    t78182
}
