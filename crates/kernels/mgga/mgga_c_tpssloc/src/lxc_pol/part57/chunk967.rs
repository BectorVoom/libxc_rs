//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 967/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk967<F: Float>(t1992: F, t22635: F, t26225: F, t6439: F, t1985: F, t28186: F, t6889: F, t6906: F, t120217: F, t120220: F, t22633: F, t32704: F, t90566: F) -> (F, F, F, F, F) {
    let t127197 = F::new(0.9869604401089358619e-1) * t1992 * t22635 * t26225 * t6439;
    let t127201 = F::new(0.16449340668482264365e-1) * t1985 * t6889 * t6906 * t28186;
    let t127202 = F::new(0.3289868133696452873e-1) * t120217;
    let t127203 = F::new(0.3289868133696452873e-1) * t120220;
    let t127210 = F::new(0.6579736267392905746e-1) * t22633 * t90566 * t32704;
    (t127197, t127201, t127202, t127203, t127210)
}
