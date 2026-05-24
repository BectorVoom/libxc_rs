//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1004/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1004<F: Float>(t1246: F, t5366: F, t1206: F, t1228: F, t13671: F, t1226: F, t1229: F, t13821: F, t13827: F, t13835: F, t13838: F, t1634: F, t1636: F, t4445: F, t4451: F, t4453: F, t4456: F, t516: F, t518: F, t5397: F, t5401: F, t5404: F) -> F {
    let t13843 = t1246 * t5366;
    let t13844 = t13843 * t1206;
    let t13847 = t1228 * t13671;
    let t13850 = -F::new(12.0) * t1226 * t5401 + F::new(3.0) * t1226 * t5404 + F::new(3.0) * t1229 * t5397 - t13821 * t518 - F::new(24.0) * t13827 * t4453 + F::new(60.0) * t13835 * t4451 - F::new(24.0) * t13838 * t4451 - F::new(12.0) * t13844 * t4451 + F::new(3.0) * t13847 * t516 + F::new(6.0) * t1634 * t4456 + F::new(6.0) * t1636 * t4445;
    t13850
}
