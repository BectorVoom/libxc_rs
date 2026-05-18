//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 730/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk730<F: Float>(t226: F, t3664: F, t773: F, t774: F, t3629: F, t783: F, t3628: F, t125: F, t1364: F, t2175: F, t1385: F, t2383: F) -> (F, F, F, F, F, F) {
    let t3665 = t3664 * t226;
    let t3667 = t773 * t774 * t3665;
    let t3670 = t3629 * t783;
    let t3671 = t3628 * t3670;
    let t3676 = t125 * t1364;
    let t3677 = t3676 * t783;
    let t3678 = t2175 * t3677;
    let t3681 = t2383 * t1385;
    (t3665, t3667, t3671, t3676, t3678, t3681)
}
