//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1139/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1139<F: Float>(t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15911: F, t15915: F, t15916: F, t15917: F, t15923: F, t15927: F, t15928: F, t9789: F, t9793: F, t9797: F) -> (F,) {
    let t16161 = -t12044 + t15911 - t12048 - t15915 - t15916 + t15917 - t12057 - t12059 + t15923 - t9789 + t12087 - t15927 - t15928 - t12094 + t9793 + t9797;
    (t16161,)
}
