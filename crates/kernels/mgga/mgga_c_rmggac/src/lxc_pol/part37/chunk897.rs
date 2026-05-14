//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 897/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk897<F: Float>(t68893: F, t74730: F, t74734: F, t74739: F, t74753: F, t77172: F, t77173: F, t77174: F, t77183: F, t77184: F, t77185: F, t77186: F, t77187: F, t77189: F, t77190: F, t77191: F, t77192: F) -> (F,) {
    let t80155 = t77172 + t77173 - t77174 - 0.17451485956252114153e-4 * t74730 + 0.34902971912504228306e-4 * t74734 + t68893 - 0.69805943825008456612e-4 * t74739 + t77183 - t77184 - t77185 + t77186 + t77187 + 0.17451485956252114153e-4 * t74753 + t77189 - t77190 + t77191 - t77192;
    (t80155,)
}
