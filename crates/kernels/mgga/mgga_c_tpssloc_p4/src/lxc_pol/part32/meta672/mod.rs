//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2105;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta672<F: Float>(t15643: F, t7345: F, t27639: F, t86264: F, t27645: F, t3540: F, t8043: F, t2136: F, t607: F, t8027: F, t1714: F, t24682: F, t460: F, t27628: F, t27634: F, t10469: F, t24719: F, t3: F, t86154: F, t2132: F, t24746: F, t1222: F, t27589: F, t1184: F, t1409: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95352, t95362, t95364, t95365, t95370, t95382, t95384) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2105::<F>(t15643, t7345, t27639, t86264, t27645, t3540, t8043, t2136, t607, t8027, t1714, t24682, t460);
        let (t95387, t95396, t95404, t95410, t95413) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106::<F>(t27628, t27634, t10469, t24719, t3, t86154, t2132, t24746, t95382, t1222, t27589, t1184, t1409);
    (t95352, t95362, t95364, t95365, t95370, t95384, t95387, t95396, t95404, t95410, t95413)
}
