//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2017/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2017<F: Float>(t15908: F, t2375: F, t12045: F, t12050: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F, t12089: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15904: F, t3734: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t9789: F, t9793: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15909 = t15908 * t2375;
    let t15910 = F::cast_from(0.10843581300301739842e-1_f64) * t15909;
    let t15911 = F::new(48.0) * t12045;
    let t15915 = F::new(24.0) * t12050;
    let t15916 = F::new(12.0) * t12052;
    let t15917 = F::new(80.0) * t12054;
    let t15921 = t5151 * t750;
    let t15923 = F::new(2.0) * t17 * t15921;
    let t15927 = F::cast_from(0.5848223622634646207e0_f64) * t12089;
    let t15928 = F::cast_from(0.34631718211362927518e2_f64) * t12091;
    let t15929 = -F::new(6.0) * t15904 * t3918 * t5161 + F::new(6.0) * t3734 * t5122 * t5126 + F::new(6.0) * t3918 * t3919 * t5187 + F::new(12.0) * t3919 * t5126 * t5308 - t12044 - t12048 - t12057 - t12059 + t12087 - t12094 + t15910 + t15911 - t15915 - t15916 + t15917 + t15923 - t15927 - t15928 - t9789 + t9793;
    (t15909, t15910, t15911, t15915, t15916, t15917, t15921, t15923, t15927, t15928, t15929)
}
