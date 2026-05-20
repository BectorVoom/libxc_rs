//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 784/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk784<F: Float>(t10140: F, t10143: F, t193: F, t202: F, t9793: F, t9797: F, t9820: F, t9824: F, t9872: F, t9876: F, t9881: F, t9884: F, t9887: F, t9890: F, t9894: F, t9896: F) -> F {
    let t10147 = F::new(2.0) * t10140 * t10143 * t193 * t202 + t9793 + t9797 - t9820 - t9824 + t9872 - t9876 + t9881 - t9884 + t9887 + t9890 - t9894 + t9896;
    t10147
}
