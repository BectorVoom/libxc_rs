//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1111/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1111<F: Float>(t76355: F, t76356: F, t76358: F, t76360: F, t76362: F, t76368: F, t76370: F, t78098: F, t78099: F, t78101: F, t78103: F, t78110: F, t78111: F) -> F {
    let t80485 = t78098 + t78099 + t78101 + t76355 + t78103 + t76356 - t76358 + t76360 + t76362 - t78110 + t78111 - t76368 + t76370;
    t80485
}
