//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 931/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk931<F: Float>(t4764: F, t5999: F, t4723: F, t5398: F, t3297: F, t136: F, t4728: F, t1113: F, t11195: F, t11204: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F) -> (F, F, F, F, F, F) {
    let t21741 = t4764 * t5999;
    let t21745 = t4723 * t5398;
    let t21746 = t3297 * t21745;
    let t21747 = t136 * t21746;
    let t21749 = t4728 * t5398;
    let t21750 = t1113 * t21749;
    let t21751 = t136 * t21750;
    let t21753 = -t11195 - F::new(0.16431333333333333333e0) * t18512 + F::new(0.19931111111111111111e0) * t18203 - F::new(0.59793333333333333333e0) * t18219 - F::new(0.29896666666666666667e0) * t18229 + F::new(0.5477111111111111111e-1) * t18494 - F::new(0.32862666666666666666e0) * t18505 - F::new(0.28483875e1) * t21739 + F::new(0.46074375e0) * t21741 - t11204 + F::new(0.39862222222222222223e0) * t14702 + F::new(0.27385555555555555556e0) * t14766 - F::new(0.82156666666666666668e-1) * t21747 + F::new(0.49293999999999999999e0) * t21751;
    (t21741, t21745, t21747, t21749, t21751, t21753)
}
