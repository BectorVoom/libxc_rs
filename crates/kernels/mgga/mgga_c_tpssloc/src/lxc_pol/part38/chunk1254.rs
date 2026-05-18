//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1254/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1254<F: Float>(t12089: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15904: F, t15910: F, t15911: F, t15915: F, t15916: F, t15917: F, t15923: F, t3734: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t9789: F, t9793: F) -> (F, F, F) {
    let t15927 = F::new(0.5848223622634646207e0) * t12089;
    let t15928 = F::new(0.34631718211362927518e2) * t12091;
    let t15929 = -F::new(6.0) * t15904 * t3918 * t5161 + F::new(6.0) * t3734 * t5122 * t5126 + F::new(6.0) * t3918 * t3919 * t5187 + F::new(12.0) * t3919 * t5126 * t5308 - t12044 - t12048 - t12057 - t12059 + t12087 - t12094 + t15910 + t15911 - t15915 - t15916 + t15917 + t15923 - t15927 - t15928 - t9789 + t9793;
    (t15927, t15928, t15929)
}
