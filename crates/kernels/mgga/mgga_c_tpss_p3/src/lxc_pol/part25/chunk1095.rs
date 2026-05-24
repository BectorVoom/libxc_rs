//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1095/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1095<F: Float>(t15107: F, t2741: F, t4978: F, t837: F, t11641: F, t11647: F, t11659: F, t11688: F, t11692: F, t11697: F, t11703: F, t15058: F, t15062: F, t15066: F, t15071: F, t15079: F, t15084: F, t15089: F, t15093: F, t15097: F, t15102: F, t2722: F, t2731: F, t2740: F, t8514: F, t8559: F, t8568: F, t9042: F, t946: F, t967: F) -> F {
    let t15108 = t2741 * t15107;
    let t15111 = t4978 * t837;
    let t15112 = t2741 * t15111;
    let t15115 = -t2731 * t15058 / F::new(3072.0) - t967 * t15062 / F::new(2304.0) + F::new(5.0) / F::new(13824.0) * t967 * t15066 - t11641 / F::new(648.0) + t11647 + t9042 + t2740 * t15071 / F::new(4608.0) + t946 * t15079 / F::new(3072.0) - t11659 + t2722 * t15084 / F::new(768.0) + t8559 * t15089 / F::new(512.0) - t8568 * t15093 / F::new(512.0) - t2740 * t15097 / F::new(1152.0) + t8514 * t15102 / F::new(1152.0) - t11688 / F::new(6912.0) - t11692 / F::new(10368.0) - t11697 + t11703 - t2740 * t15108 / F::new(2304.0) + t8514 * t15112 / F::new(2304.0);
    t15115
}
