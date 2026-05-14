//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1339/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1339<F: Float>(t68399: F, t68439: F, t68474: F, t68525: F, t1114: F, t1143: F, t1148: F, t12554: F, t12569: F, t12573: F, t12637: F, t1586: F, t1705: F, t1887: F, t19103: F, t19106: F, t19115: F, t19123: F, t19129: F, t19135: F, t19158: F, t19159: F, t19162: F, t20853: F, t20863: F, t20865: F, t20877: F, t20891: F, t20893: F, t20910: F, t3119: F, t3144: F, t4323: F, t473: F, t6019: F, t6024: F, t6025: F, t6031: F, t6038: F, t63208: F, t63237: F, t63357: F, t63392: F, t6509: F, t6514: F, t6517: F, t6521: F, t68280: F, t68321: F, t68356: F, t935: F) -> (F, F) {
    let t68527 = t68399 + t68439 + t68474 + t68525;
    let t68530 = 2.0 * t19129 * t20891 * t1143 * t3144 + 2.0 * t6024 * t6025 * t19103 * t1586 - 2.0 * t20863 * t6038 - 4.0 * t68321 * t68280 * t1114 * t1148 - t1705 * t12554 * t935 * t1887 + 4.0 * t63237 * t20893 + 4.0 * t20865 * t19135 + 2.0 * t63208 * t6517 - t6031 * t63392 * t6521 + 4.0 * t6024 * t6025 * t20853 * t1148 - 2.0 * t19106 * t4323 + 4.0 * t19115 * t20910 - 2.0 * t63357 * t20877 * t19158 - t6514 * t19162 - t6019 * t12637 - 6.0 * t6019 * t12569 - 6.0 * t6024 * t19123 * t6509 * t3119 + t68356 * t19159 + 4.0 * t6019 * t12573 + param_beta * t68527 * t473;
    (t68527, t68530)
}
