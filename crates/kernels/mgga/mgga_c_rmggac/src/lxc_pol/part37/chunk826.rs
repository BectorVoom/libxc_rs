//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 826/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk826<F: Float>(t13848: F, t13850: F, t8688: F, t2314: F, t68658: F, t14363: F, t15231: F, t13996: F, t2868: F, t11723: F, t69041: F, t14236: F, t2078: F, t3369: F, t56399: F) -> (F, F, F, F, F, F) {
    let t74861 = t8688 * t13848 * t13850;
    let t74864 = t2314 * t68658 * t13850;
    let t74867 = t14363 * t15231;
    let t74870 = F::cast_from(0.2993560425465952141e-1_f64) * t2868 * t13996;
    let t74873 = t69041 * t11723;
    let t74889 = t14236 * t3369 * t2078 * t56399;
    (t74861, t74864, t74867, t74870, t74873, t74889)
}
