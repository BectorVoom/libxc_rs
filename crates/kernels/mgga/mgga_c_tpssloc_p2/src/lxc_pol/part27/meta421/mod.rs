//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1729;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta421<F: Float>(t15904: F, t8643: F, t22574: F, t3701: F, t3914: F, t2019: F, t1983: F, t6996: F, t6999: F, t1390: F, t3719: F, t6878: F, t1266: F, t1393: F, t1869: F, t1976: F, t1980: F, t22460: F, t22461: F, t22467: F, t22482: F, t22483: F, t22559: F, t22563: F, t2314: F, t2320: F, t2323: F, t3652: F, t3929: F, t510: F, t650: F, t6515: F, t6517: F, t652: F, t6539: F, t672: F, t6862: F, t6872: F) -> (F, F, F, F, F, F, F) {
        let (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1729::<F>(t15904, t8643, t22574, t3701, t3914, t2019, t1983, t6996, t6999, t1390, t3719, t6878);
        let t22588 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730::<F>(t1983, t22585, t1266, t1393, t1869, t1976, t1980, t22460, t22461, t22467, t22482, t22483, t22559, t22563, t22577, t22580, t22583, t2314, t2320, t2323, t3652, t3929, t510, t650, t6515, t6517, t652, t6539, t672, t6862, t6872);
    (t22575, t22578, t22579, t22581, t22584, t22585, t22588)
}
