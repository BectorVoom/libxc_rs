//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1361/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1361<F: Float>(t73315: F, t73333: F, t73353: F, t73375: F, t73396: F, t73419: F, t73442: F, t73459: F, t1587: F, t15930: F, t15944: F, t15953: F, t1705: F, t1884: F, t1885: F, t1887: F, t19115: F, t19129: F, t19143: F, t19150: F, t20863: F, t20865: F, t20868: F, t20887: F, t20892: F, t20910: F, t20914: F, t22055: F, t22058: F, t22078: F, t4314: F, t452: F, t473: F, t6019: F, t6031: F, t6034: F, t63219: F, t63237: F, t6521: F, t6525: F, t68192: F, t68224: F, t68321: F, t68581: F, t68585: F, t73264: F, t73278: F, t73285: F, t73289: F, t935: F) -> (F,) {
    let t73462 = t73315 + t73333 + t73353 + t73375 + t73396 + t73419 + t73442 + t73459;
    let t73474 = -t6031 * t19150 * t22078 + 4.0 * t63237 * t22058 + 4.0 * t19129 * t68581 * t6521 - 2.0 * t20868 * t20914 - 12.0 * t68224 * t73264 * t20892 - t1705 * t15930 * t935 * t1887 + 2.0 * t6019 * t15953 + 4.0 * t19129 * t68585 * t6521 - 6.0 * t19115 * t22055 - 6.0 * t63219 * t73278 * t6034 - 4.0 * t68321 * t73264 * t4314 + 2.0 * t19143 * t73285 * t4314 + 2.0 * t19129 * t73289 * t20892 - 2.0 * t68192 * t1587 - 6.0 * t6019 * t15944 - t1884 * t1885 * t452 * t73462 + 4.0 * t20865 * t20910 + 4.0 * t20865 * t20887 + param_beta * t73462 * t473 - 2.0 * t20863 * t6525;
    (t73474,)
}
