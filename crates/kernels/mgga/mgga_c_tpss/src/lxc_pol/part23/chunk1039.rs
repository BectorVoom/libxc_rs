//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1039/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1039<F: Float>(t1482: F, t2776: F, t2804: F, t2723: F, t9081: F, t9095: F, t1464: F, t2768: F, t3949: F, t975: F, t11436: F, t366: F, t2703: F, t2785: F, t345: F, t9080: F) -> (F, F, F, F, F, F, F, F) {
    let t11730 = t2776 * t1482 * t2804;
    let t11733 = t9081 * t2723;
    let t11743 = t9095 * t2723;
    let t11750 = t2768 * t1464;
    let t11753 = t975 * t3949;
    let t11760 = t366 * t11436;
    let t11767 = t2785 * t2703 * t345;
    let t11771 = t9080 * t2723 * t345;
    (t11730, t11733, t11743, t11750, t11753, t11760, t11767, t11771)
}
