//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1333/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1333<F: Float>(t1107: F, t28778: F, t68276: F, t19142: F, t6513: F, t12404: F, t6013: F, t12463: F, t19090: F, t12478: F, t12407: F, t19077: F, t12359: F, t20831: F, t3062: F, t12445: F, t6007: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68321 = t68276 * t28778 * t1107;
    let t68356 = t6513 * t19142;
    let t68361 = 5.0 / 5184.0 * t6013 * t12404;
    let t68365 = t19090 * t12463 / 1152.0;
    let t68373 = t6013 * t12478 / 864.0;
    let t68387 = t19077 * t12407 / 576.0;
    let t68391 = t6013 * t12359 / 1728.0;
    let t68393 = t20831 * t3062 / 216.0;
    let t68394 = t6007 * t12445;
    (t68321, t68356, t68361, t68365, t68373, t68387, t68391, t68393, t68394)
}
