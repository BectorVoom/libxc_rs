//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1058/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1058<F: Float>(t10029: F, t1614: F, t3211: F, t3214: F, t1170: F, t4430: F, t1173: F, t4377: F, t724: F, t489: F, t10033: F, t2215: F, t4438: F, t2206: F, t10039: F, t10028: F, t10038: F, t10042: F, t7979: F, t7988: F, t7992: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12907 = 0.11696447245269292414e1 * t10029;
    let t12908 = t3211 * t1614;
    let t12909 = 12.0 * t12908;
    let t12910 = t3214 * t1614;
    let t12911 = 32.0 * t12910;
    let t12913 = 8.0 * t1170 * t4430;
    let t12915 = 8.0 * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = 2.0 * t489 * t12916;
    let t12919 = 40.0 * t10033;
    let t12920 = t4438 * t2215;
    let t12921 = 0.17315859105681463759e2 * t12920;
    let t12922 = t4438 * t2206;
    let t12923 = 0.5848223622634646207e0 * t12922;
    let t12924 = 4.0 * t10039;
    let t12925 = -t10028 - t12907 + t7979 + t12909 - t12911 + t12913 - t12915 + t12918 + t12919 - t12921 - t12923 - t10038 + t12924 - t10042 + t7988 + t7992;
    (t12907, t12909, t12911, t12913, t12915, t12918, t12919, t12921, t12923, t12924, t12925)
}
