//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1352/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1352<F: Float>(t219: F, t22038: F, t22045: F, t6030: F, t1148: F, t1149: F, t15796: F, t1586: F, t15948: F, t16013: F, t19115: F, t19118: F, t19143: F, t19150: F, t20853: F, t20865: F, t20883: F, t20893: F, t20897: F, t20906: F, t22037: F, t22066: F, t22074: F, t22079: F, t22081: F, t22082: F, t342: F, t450: F, t5294: F, t6016: F, t6019: F, t6024: F, t6025: F, t6031: F, t6032: F, t6035: F, t63371: F, t6517: F, t6521: F, t6522: F, t68235: F, t68273: F, t68356: F, t68532: F, t68572: F) -> (F,) {
    let t73211 = t22038 * t219;
    let t73219 = t22045 * t6030;
    let t73253 = -t6019 * t16013 + t19143 * t19150 * t22081 + t63371 * t22082 - t73211 * t1149 + 4.0 * t68273 * t6517 - 2.0 * t6031 * t68235 * t6521 - t19118 * t22079 - t73219 * t6035 + 4.0 * t20865 * t20883 - t6031 * t6032 * t15796 * t342 * t450 - 2.0 * t19118 * t22074 - 2.0 * t68572 * t6522 + 4.0 * t6024 * t6025 * t20853 * t1586 + 4.0 * t68532 * t20893 - 4.0 * t68356 * t20897 + 2.0 * t68356 * t20906 + 2.0 * t6024 * t6025 * t22037 * t1148 + 2.0 * t6024 * t6025 * t6016 * t5294 + 2.0 * t19115 * t22066 + 4.0 * t6019 * t15948;
    (t73253,)
}
