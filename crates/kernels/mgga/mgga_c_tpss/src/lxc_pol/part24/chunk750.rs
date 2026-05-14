//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 750/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk750<F: Float>(t2724: F, t4977: F, t947: F, t242: F, t1407: F, t1465: F, t2741: F, t4840: F, t4842: F, t4846: F, t4878: F, t4881: F, t4947: F, t4949: F, t4951: F, t4955: F, t4959: F, t4963: F) -> (F, F, F, F, F) {
    let t4978 = t4977 * t2724;
    let t4979 = t947 * t4978;
    let t4980 = t242 * t4979;
    let t4984 = t1465 * t1407;
    let t4985 = t2741 * t4984;
    let t4988 = -t4840 + t4842 - t4846 + t4878 + t4881 + t4947 + t4949 - t4951 + t4955 - t4959 - t4963;
    (t4978, t4980, t4984, t4985, t4988)
}
