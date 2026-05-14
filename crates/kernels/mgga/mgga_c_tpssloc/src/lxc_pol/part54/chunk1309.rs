//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1309/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1309<F: Float>(t1851: F, t8852: F, t117407: F, t117410: F, t117412: F, t117416: F, t117420: F, t117422: F, t124673: F, t124687: F, t125024: F, t125029: F, t125043: F, t125046: F, t125050: F, t1398: F, t1852: F, t1858: F, t2099: F, t2170: F, t27286: F, t27930: F, t3: F, t32393: F, t32415: F, t580: F, t7416: F, t7961: F) -> (F,) {
    let t125053 = t1851 * t8852;
    let t125058 = t117422 + t2099 * t27930 + t124673 + t1398 * (t124687 + t125029 + t125043 + t125046) + t125050 + t117410 + t117416 + t32393 * t1858 + t117412 + t1852 * t32415 + t125053 + t117420 + t117407 + t2170 * t27286 + t3 * t125024 * t580 + t7416 * t7961;
    (t125058,)
}
