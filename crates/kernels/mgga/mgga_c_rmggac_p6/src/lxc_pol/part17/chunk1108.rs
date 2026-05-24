//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1108/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1108<F: Float>(t10198: F, t42335: F, t42336: F, t42337: F, t42338: F, t42339: F, t42340: F, t42341: F, t42345: F, t7938: F, t7941: F, t10201: F, t10204: F, t10206: F, t42355: F, t42356: F, t42357: F, t42358: F, t42359: F, t9231: F, t9671: F, t9672: F) -> (F, F) {
    let t48086 = -t42335 + t42336 + t42337 + t42338 + t10198 - t42339 - t42340 + t42341 + t7938 - t7941 - t42345;
    let t48091 = t9671 + t10201 - t42355 - t9672 + F::new(4.0) * t9231 + t10204 - t42356 - t42357 - t42358 + t42359 - t10206;
    (t48086, t48091)
}
