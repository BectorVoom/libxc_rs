//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 708/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk708<F: Float>(t4459: F, t520: F, t1224: F, t774: F, t1233: F, t4416: F, t4415: F, t125: F, t1625: F, t3273: F, t1646: F, t3342: F, t1206: F) -> (F, F, F, F, F, F) {
    let t4460 = t4459 * t520;
    let t4462 = t1224 * t774 * t4460;
    let t4465 = t4416 * t1233;
    let t4466 = t4415 * t4465;
    let t4471 = t125 * t1625;
    let t4472 = t4471 * t1233;
    let t4473 = t3273 * t4472;
    let t4476 = t3342 * t1646;
    let t4478 = t1625 * t1206;
    (t4460, t4462, t4466, t4473, t4476, t4478)
}
