//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1242/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1242<F: Float>(t22069: F, t3054: F, t19144: F, t20913: F, t6521: F, t342: F, t450: F, t5242: F, t6032: F, t1885: F, t22037: F, t452: F, t1587: F, t1884: F, t1887: F, t19129: F, t19143: F, t20856: F, t20865: F, t20868: F, t22038: F, t22046: F, t22055: F, t22058: F, t22062: F, t22066: F, t473: F, t5276: F, t5295: F, t6019: F, t6024: F, t6031: F, t6514: F, t6517: F, t6522: F, t6525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22070 = t22069 * t3054;
    let t22071 = t19144 * t22070;
    let t22074 = t20913 * t6521;
    let t22078 = t5242 * t342 * t450;
    let t22079 = t6032 * t22078;
    let t22081 = t22069 * t450;
    let t22082 = t6032 * t22081;
    let t22085 = t1885 * t452 * t22037;
    let t22087 = -2.0 * t1587 * t20856 - t1884 * t22085 - t1887 * t22046 + 4.0 * t19129 * t22058 - 2.0 * t19143 * t22071 + t19143 * t22082 + 4.0 * t20865 * t6517 - 2.0 * t20868 * t6522 + t22038 * t473 - 6.0 * t22055 * t6024 + 4.0 * t22062 * t6024 + 2.0 * t22066 * t6024 - 2.0 * t22074 * t6031 - t22079 * t6031 + 2.0 * t5276 * t6019 - t5295 * t6019 - 2.0 * t6514 * t6525;
    (t22070, t22071, t22074, t22078, t22079, t22081, t22082, t22085, t22087)
}
