//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1309/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1309<F: Float>(t14171: F, t17964: F, t14185: F, t61025: F, t69945: F, t69948: F, t69950: F, t69952: F, t69954: F, t69956: F, t69958: F, t69960: F, t69962: F, t69964: F, t14304: F, t5547: F) -> (F, F) {
    let t69966 = t17964 * t14171;
    let t69968 = t17964 * t14185;
    let t69970 = -t69945 / 4.0 + t69948 / 8.0 - t61025 - t69950 / 384.0 + 7.0 / 576.0 * t69952 - 35.0 / 576.0 * t69954 - t69956 / 768.0 - t69958 / 1536.0 - 5.0 / 384.0 * t69960 - t69962 / 256.0 + t69964 / 256.0 + t69966 / 384.0 - t69968 / 1536.0;
    let t69972 = t5547 * t14304;
    (t69970, t69972)
}
