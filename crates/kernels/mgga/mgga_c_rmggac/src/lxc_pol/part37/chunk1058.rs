//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1058/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1058<F: Float>(t71005: F, t74553: F, t74557: F, t74569: F, t74587: F, t77087: F, t77090: F, t77093: F, t77096: F, t77099: F, t77105: F, t77107: F, t77108: F, t77109: F, t77110: F, t77111: F, t77113: F) -> F {
    let t80135 = -t77087 - t77090 - t77093 + t77096 + t77099 - t71005 - F::new(0.17451485956252114153e-4) * t74553 + F::new(0.17451485956252114153e-4) * t74557 + t77105 + F::new(0.10511583655740820312e-5) * t74569 + t77107 - t77108 - t77109 - t77110 - t77111 + F::new(0.17519306092901367187e-6) * t74587 - t77113;
    t80135
}
