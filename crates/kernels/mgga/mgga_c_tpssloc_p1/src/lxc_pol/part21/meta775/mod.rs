//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2683;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta775<F: Float>(t39510: F, t39512: F, t39514: F, t39522: F, t39530: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39532: F, t19572: F, t67: F, t758: F, t39540: F, t54428: F, t16018: F, t16490: F, t193: F, t19924: F, t20093: F, t3918: F, t3919: F, t39539: F, t39549: F, t39563: F, t5122: F, t5126: F, t55224: F, t6347: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t56365, t56366, t56367, t56368, t56369, t56370) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2683::<F>(t39510, t39512, t39514, t39522, t39530, t39499, t39502, t39505, t39508, t39518, t39521, t39529);
        let (t56372, t56375, t56381, t56388, t56389) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684::<F>(t39532, t19572, t67, t758, t39540, t54428, t16018, t16490, t193, t19924, t20093, t3918, t3919, t39539, t39549, t39563, t5122, t5126, t55224, t6347);
    (t56365, t56366, t56367, t56368, t56369, t56370, t56372, t56375, t56381, t56388, t56389)
}
