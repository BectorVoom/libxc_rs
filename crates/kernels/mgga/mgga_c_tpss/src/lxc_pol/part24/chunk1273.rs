//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1273/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1273<F: Float>(t13565: F, t19315: F, t5536: F, t6103: F, t68913: F, t68915: F, t68917: F, t68919: F, t68921: F, t68923: F, t68927: F, t68929: F, t68931: F, t68934: F, t68936: F, t68941: F, t68944: F, t68946: F, t68949: F, t68953: F, t68956: F) -> (F,) {
    let t68957 = -2.0 * t13565 * t5536 - 4.0 * t19315 * t6103 - t68913 - t68915 - t68917 - t68919 - t68921 - t68923 - t68927 - t68929 - t68931 - t68934 - t68936 - t68941 + t68944 + t68946 - t68949 - t68953 + t68956;
    (t68957,)
}
