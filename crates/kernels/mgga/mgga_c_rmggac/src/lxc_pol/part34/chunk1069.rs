//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1069/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1069<F: Float>(t78367: F, t75907: F, t75910: F, t15605: F, t302: F, t70063: F, t70100: F, t71670: F, t71671: F, t71672: F, t72: F, t75892: F, t75895: F, t78349: F, t78352: F, t78355: F, t78359: F, t78362: F, t78364: F) -> F {
    let t78368 = F::new(0.42564599893297839398e-5) * t78367;
    let t78371 = F::new(0.1276937996798935182e-4) * t75907;
    let t78372 = F::new(0.1276937996798935182e-4) * t75910;
    let t78373 = -t78349 - t78352 - t78355 + F::new(0.16566831523319392755e-1) * t75892 - F::new(0.91976356987732177731e-5) * t70063 - F::new(0.20439190441718261719e-5) * t75895 - t71670 - t71671 - t71672 + t78359 - F::new(0.15372131649401827111e-4) * t70100 + t78362 + t78364 - t78368 + t72 * t302 * t15605 + t78371 - t78372;
    t78373
}
