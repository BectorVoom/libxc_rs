//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1930;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta615<F: Float>(t26308: F, t3777: F, t5252: F, t16257: F, t26309: F, t5293: F, t80820: F, t5259: F, t80816: F, t16244: F, t22833: F, t5303: F, t16366: F, t16370: F, t26257: F, t3872: F, t1831: F, t80869: F, t22783: F, t5314: F, t26297: F, t80853: F, t80855: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91116, t91118, t91120, t91122, t91124, t91126) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1930::<F>(t26308, t3777, t5252, t16257, t26309, t5293, t80820, t5259, t80816, t16244, t22833, t5303);
        let (t91128, t91130, t91133, t91135, t91137, t91140) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1931::<F>(t16366, t22833, t16370, t26257, t3872, t1831, t80869, t22783, t5314, t26297, t80853, t80855);
    (t91116, t91118, t91120, t91122, t91124, t91126, t91128, t91130, t91133, t91135, t91137, t91140)
}
