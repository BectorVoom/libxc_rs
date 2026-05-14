//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 891/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk891<F: Float>(t68614: F, t74491: F, t74495: F, t74506: F, t77031: F, t77034: F, t77037: F, t77043: F, t77046: F, t77049: F, t77052: F, t77054: F, t77055: F, t77056: F, t77058: F, t77061: F, t77062: F) -> (F,) {
    let t80113 = t77031 + t77034 - t77037 - t74491 - 0.40992351065071538967e-4 * t68614 - t77043 + t77046 - t77049 - t77052 - 0.1313947956967602539e-5 * t74495 + t77054 - t77055 + t77056 - t77058 - t77061 + t77062 + t74506;
    (t80113,)
}
