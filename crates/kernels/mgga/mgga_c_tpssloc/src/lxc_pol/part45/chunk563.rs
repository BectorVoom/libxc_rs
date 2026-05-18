//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 563/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk563<F: Float>(t240: F, t6612: F, t812: F, t831: F, t1899: F, t838: F, t234: F, t59: F, t849: F, t6580: F, t6582: F, t6587: F, t6594: F, t6603: F, t6607: F, t6610: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6613 = t6612 * t240;
    let t6614 = t812 * t6613;
    let t6615 = t6614 * t831;
    let t6617 = t1899 * t838;
    let t6618 = F::new(7.0) / F::new(2304.0) * t6617;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    let t6622 = t6621 * t849;
    let t6624 = -t6580 - t6582 / F::new(48.0) - t6587 - F::new(0.12111826828242117256e-2) * t6594 - t6603 - F::new(0.20186378047070195427e-3) * t6607 + t6610 / F::new(1536.0) - t6615 / F::new(1536.0) - t6618 - t6622 / F::new(384.0);
    (t6613, t6614, t6615, t6617, t6619, t6620, t6621, t6622, t6624)
}
