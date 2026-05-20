//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1629;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta423<F: Float>(t19164: F, t19207: F, t1241: F, t1235: F, t6150: F, t1760: F, t5088: F, t3598: F, t1251: F, t6267: F, t6243: F, t11606: F, t1238: F, t15820: F, t1761: F, t18287: F, t19121: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5060: F, t6268: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19208, t19209, t19211, t19213, t19214, t19219, t19220, t19225, t19226) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1629::<F>(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1630::<F>(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
    (t19208, t19209, t19211, t19213, t19214, t19219, t19220, t19225, t19226, t19231)
}
