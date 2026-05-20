//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1917;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1918;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1919;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta534<F: Float>(t26309: F, t5252: F, t22833: F, t5293: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F, t1339: F, t22856: F, t22859: F, t22860: F, t22864: F, t22868: F, t26306: F, t22766: F, t22780: F, t22798: F, t22805: F, t22820: F, t22826: F, t26231: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26251: F, t26280: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t539: F, t1887: F, t22839: F) -> (F, F, F, F, F, F, F) {
        let (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1917::<F>(t26309, t5252, t22833, t5293, t5303, t1351, t16311, t3788, t6936, t16306, t550, t1339);
        let t26326 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1918::<F>(t26323, t6936, t22856, t22859, t22860, t22864, t22868, t26306, t26310, t26312, t26314, t26320);
        let t26328 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1919::<F>(t22766, t22780, t22798, t22805, t22820, t22826, t26231, t26234, t26236, t26238, t26240, t26246, t26249, t26251, t26280, t26286, t26290, t26293, t26295, t26299, t26303, t26326);
        let (t26329, t26331) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1920::<F>(t26328, t539, t1887, t22839);
    (t26318, t26319, t26322, t26323, t26328, t26329, t26331)
}
