//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1426/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1426<F: Float>(t15908: F, t2375: F, t12045: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F, t1787: F, t2516: F, t12120: F, t2663: F, t5157: F) -> (F, F, F, F, F, F, F, F) {
    let t15909 = t15908 * t2375;
    let t15911 = F::new(48.0) * t12045;
    let t15916 = F::new(12.0) * t12052;
    let t15917 = F::new(80.0) * t12054;
    let t15921 = t5151 * t750;
    let t15923 = F::new(2.0) * t17 * t15921;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15976 = F::new(4.0) * t12120;
    let t15979 = t5157 * t2663;
    (t15909, t15911, t15916, t15917, t15923, t15972, t15976, t15979)
}
