//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1076/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1076<F: Float>(t1089: F, t11830: F, t2998: F, t4180: F, t4206: F, t4205: F, t9384: F, t4101: F, t673: F, t1014: F, t10353: F, t1038: F) -> (F, F, F, F, F, F, F) {
    let t11832 = F::cast_from(0.23392894490538584828e1_f64) * t1089 * t11830;
    let t11833 = t2998 * t4180;
    let t11834 = t11833 * t4206;
    let t11836 = F::cast_from(0.34631718211362927518e2_f64) * t1089 * t11834;
    let t11837 = t4205 * t9384;
    let t11839 = F::cast_from(0.17315859105681463759e2_f64) * t1089 * t11837;
    let t11844 = t673 * t4101;
    let t11845 = F::cast_from(0.10954222222222222222e0_f64) * t11844;
    let t11846 = t1014 * t10353;
    let t11847 = t1038 * t11846;
    (t11832, t11836, t11839, t11844, t11845, t11846, t11847)
}
