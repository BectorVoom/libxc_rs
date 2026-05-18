//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1050/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1050<F: Float>(t73234: F, t74197: F, t74199: F, t74207: F, t74209: F, t74213: F, t74217: F, t74225: F, t76897: F, t76898: F, t76904: F, t76913: F, t76918: F, t76923: F, t76925: F, t76927: F, t76928: F) -> F {
    let t80066 = -t76897 - t76898 - F::new(0.57000320883372412499e-7) * t74197 - F::new(0.57000320883372412499e-7) * t74199 + t76904 + F::new(0.58171619854173713844e-5) * t74207 - F::new(0.58171619854173713844e-5) * t74209 + F::new(0.58171619854173713844e-5) * t74213 - F::new(0.17451485956252114153e-4) * t74217 - t76913 - F::new(0.2363e1) * t73234 + t76918 + t76923 + t76925 - t76927 + t76928 - t74225;
    t80066
}
