//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1336/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1336<F: Float>(t13547: F, t16037: F, t1865: F, t20953: F, t3493: F, t4541: F, t5986: F, t6544: F, t68927: F, t68929: F, t68931: F, t68934: F, t68936: F, t68941: F, t68944: F, t68946: F, t68949: F, t68953: F, t68956: F, t68961: F, t68969: F, t68973: F, t68976: F) -> (F,) {
    let t72827 = -2.0 * t13547 * t5986 - t16037 * t1865 - 4.0 * t20953 * t3493 + 2.0 * t4541 * t6544 - t68927 - t68929 - t68931 - t68934 - t68936 - t68941 + t68944 + t68946 - t68949 - t68953 + t68956 + t68961 + t68969 + t68973 + t68976;
    (t72827,)
}
