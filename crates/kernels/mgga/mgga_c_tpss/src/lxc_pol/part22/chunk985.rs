//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 985/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk985<F: Float>(t10578: F, t10579: F, t2177: F, t1378: F, t2162: F, t750: F, t782: F, t125: F, t3664: F, t3628: F, t783: F, t2365: F, t3629: F) -> (F, F, F, F, F, F) {
    let t10581 = t10578 * t10579 * t2177;
    let t10584 = t1378 * t2162;
    let t10585 = t782 * t750;
    let t10587 = t10578 * t10584 * t10585;
    let t10590 = t125 * t3664;
    let t10592 = t3628 * t10590 * t783;
    let t10596 = t3628 * t3629 * t2365;
    (t10581, t10584, t10587, t10590, t10592, t10596)
}
