//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1903/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903<F: Float>(t1842: F, t22633: F, t22635: F, t90516: F, t1992: F, t26355: F, t90566: F, t1307: F, t26331: F, t567: F, t6347: F, t1985: F, t20022: F, t6889: F, t6906: F) -> (F, F, F, F) {
    let t97644 = t22633 * t22635 * t90516 * t1842;
    let t97647 = t1992 * t90566 * t26355;
    let t97652 = t26331 * t22635 * t567 * t6347 * t1307;
    let t97658 = t1985 * t6889 * t6906 * t20022;
    (t97644, t97647, t97652, t97658)
}
