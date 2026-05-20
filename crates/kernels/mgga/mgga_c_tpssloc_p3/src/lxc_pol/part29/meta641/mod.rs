//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2109;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2110;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2111;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta641<F: Float>(t4191: F, t81865: F, t13302: F, t23146: F, t13322: F, t4250: F, t13316: F, t13312: F, t81749: F, t23145: F, t4166: F, t2649: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F, t25132: F, t81876: F, t13336: F, t1898: F, t249: F, t23047: F, t2635: F, t81736: F, t81743: F, t81750: F, t87183: F, t1516: F, t81766: F, t23127: F, t4261: F, t13347: F, t6621: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F, t25093: F) -> (F, F, F, F, F, F, F, F) {
        let (t87185, t87187, t87189, t87191, t87193, t87195, t87198, t87200) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2109::<F>(t4191, t81865, t13302, t23146, t13322, t4250, t13316, t13312, t81749, t23145, t4166, t2649);
        let (t87202, t87206, t87212, t87213) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2110::<F>(t22690, t234, t7496, t776, t81792, t23109, t23110, t232, t236, t4233, t25132, t81876);
        let t87221 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2111::<F>(t13336, t1898, t249, t23047, t4166, t2635, t81736, t81743, t81750, t87183, t87185, t87187, t87189, t87191, t87193, t87195, t87198, t87200, t87206, t87212, t87213);
        let (t87222, t87224, t87226, t87229, t87230, t87233) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2112::<F>(t1516, t81766, t23127, t4261, t13347, t6621, t131, t6598, t9537, t225, t2627, t236, t25093);
    (t87202, t87221, t87222, t87224, t87226, t87229, t87230, t87233)
}
