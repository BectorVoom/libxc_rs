//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 929/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk929<F: Float>(t75419: F, t75421: F, t75425: F, t78165: F, t78166: F, t78167: F, t78168: F, t78169: F, t78170: F, t78171: F, t78172: F, t78173: F, t78174: F, t78175: F, t78176: F, t78179: F, t78181: F) -> (F,) {
    let t80370 = -t78165 + t78166 + t78167 - t78168 + t78169 + t78170 - t78171 + t78172 - t78173 - t78174 - t78175 - t78176 - 0.45360193192290319575e-3 * t75419 + 0.63504270469206447405e-3 * t75421 + t78179 - 0.19286482142499735879e-3 * t75425 - t78181;
    (t80370,)
}
