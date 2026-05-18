//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1212/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1212<F: Float>(t2776: F, t6785: F, t6784: F, t1003: F, t1058: F, t1953: F, t23346: F, t23601: F, t23666: F, t23670: F, t23674: F, t23680: F, t23687: F, t23693: F, t23698: F, t23701: F, t23705: F, t23707: F, t23712: F, t3076: F, t3186: F, t353: F, t6680: F, t6687: F, t6787: F, t6790: F, t6797: F, t6802: F, t6806: F, t6813: F) -> (F, F, F) {
    let t23714 = t6785 * t2776;
    let t23715 = t6784 * t23714;
    let t23720 = F::new(0.54831135561607547884e-2) * t23666 - F::new(0.43864908449286038306e-1) * t23670 * t6802 + F::new(0.82246703342411321825e-2) * t6797 * t23674 + F::new(0.16449340668482264365e-1) * t23601 * t23680 - F::new(0.43864908449286038306e-1) * t6680 * t6806 + F::new(0.54831135561607547884e-2) * t6687 * t23687 - F::new(0.14621636149762012769e-1) * t23346 * t6787 + F::new(0.27415567780803773942e-2) * t6687 * t23693 + F::new(0.36554090374405031923e-2) * t6687 * t23698 + F::new(2.0) * t3186 * t23701 + t1058 * t23705 + t353 * t23707 + t3076 * t1953 + F::new(2.0) * t1003 * t6813 + F::new(0.18277045187202515961e-2) * t23712 - F::new(0.54831135561607547884e-2) * t6687 * t23715 + F::new(0.43864908449286038306e-1) * t23346 * t6790;
    (t23714, t23715, t23720)
}
