//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1110/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1110<F: Float>(t12957: F, t520: F, t1224: F, t774: F, t10141: F, t1222: F, t12858: F, t12861: F, t12865: F, t12869: F, t12873: F, t12877: F, t12881: F, t12883: F, t12889: F, t12891: F, t12894: F, t12898: F, t12902: F, t3271: F, t4413: F) -> (F, F, F) {
    let t12958 = t12957 * t520;
    let t12960 = t1224 * t774 * t12958;
    let t12963 = -t10141 * t12858 / 4.0 - 35.0 / 216.0 * t12861 + t3271 * t12865 / 384.0 + t4413 * t12869 / 768.0 + t3271 * t12873 / 768.0 - t3271 * t12877 / 3072.0 - t12881 - 5.0 / 768.0 * t3271 * t12883 - t12889 - t12891 * t12894 / 512.0 + t4413 * t12898 / 512.0 + t12902 - t1222 * t12960 / 3072.0;
    (t12958, t12960, t12963)
}
