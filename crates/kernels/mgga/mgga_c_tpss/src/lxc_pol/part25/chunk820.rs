//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 820/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk820<F: Float>(t6419: F, t1656: F, t1838: F, t5740: F, t1639: F, t520: F, t5745: F, t1773: F, t522: F, t1657: F, t1772: F, t1842: F, t538: F, t5739: F, t5921: F, t6260: F) -> (F, F, F, F, F, F) {
    let t6420 = param_beta * t6419;
    let t6424 = t1838 * t1656;
    let t6425 = t5740 * t6424;
    let t6430 = t5745 * t1838 * t1639 * t520;
    let t6433 = t1773 * t522 * t6419;
    let t6435 = -t1657 * t5921 - t1772 * t6433 - t1842 * t6260 + t538 * t6420 + 2.0 * t5739 * t6425 + t5739 * t6430;
    (t6420, t6424, t6425, t6430, t6433, t6435)
}
