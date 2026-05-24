//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1152/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1152<F: Float>(t10318: F, t10319: F, t10320: F, t10321: F, t42459: F, t42460: F, t42461: F, t42462: F, t42463: F, t42464: F, t42465: F, t10325: F, t10329: F, t10330: F, t42474: F, t42475: F, t42476: F, t42477: F, t42478: F, t42484: F, t42485: F, t42486: F) -> (F, F) {
    let t49842 = -t10318 - t42459 + t10319 - t10320 - t10321 - t42460 + t42461 + t42462 - t42463 - t42464 - t42465;
    let t49845 = -t42474 + t42475 + t42476 + t42477 - t42478 - t42484 - t42485 - t42486 + t10325 + t10329 + t10330;
    (t49842, t49845)
}
