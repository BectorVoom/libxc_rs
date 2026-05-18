//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 615/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk615<F: Float>(t15517: F, t2412: F, t3219: F, t1986: F, t2472: F, t675: F, t2471: F, t36: F, t739: F, t15281: F, t2211: F, t2367: F) -> (F, F, F, F, F, F, F, F) {
    let t15518 = F::new(0.39914139006212695214e-1) * t15517;
    let t15521 = t2412 * t3219;
    let t15522 = F::new(0.42564599893297839398e-5) * t15521;
    let t15523 = t1986 * t2472;
    let t15524 = t675 * t15523;
    let t15525 = F::new(0.42564599893297839398e-5) * t15524;
    let t15526 = t2471 * t36;
    let t15527 = t739 * t15526;
    let t15528 = F::new(0.14967802127329760705e-1) * t15527;
    let t15529 = F::new(0.14967802127329760705e-1) * t15281;
    let t15530 = t2211 * t2367;
    (t15518, t15522, t15523, t15525, t15526, t15528, t15529, t15530)
}
