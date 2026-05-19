//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1209/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1209<F: Float>(t1307: F, t1388: F, t118: F, t1787: F, t2375: F, t12045: F, t12050: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F) -> (F, F, F, F, F, F, F) {
    let t15904 = t1388 * t1307;
    let t15908 = t1787 * t118;
    let t15909 = t15908 * t2375;
    let t15910 = F::cast_from(0.10843581300301739842e-1_f64) * t15909;
    let t15911 = F::new(48.0) * t12045;
    let t15915 = F::new(24.0) * t12050;
    let t15916 = F::new(12.0) * t12052;
    let t15917 = F::new(80.0) * t12054;
    let t15921 = t5151 * t750;
    let t15923 = F::new(2.0) * t17 * t15921;
    (t15904, t15910, t15911, t15915, t15916, t15917, t15923)
}
