//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1050/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1050<F: Float>(t14792: F, t885: F, t11289: F, t11366: F, t1437: F, t14658: F, t14685: F, t14734: F, t14739: F, t2545: F, t2589: F, t3822: F, t3827: F, t3845: F, t4892: F, t4908: F, t4911: F, t4940: F, t4943: F, t877: F, t8842: F, t886: F, t8899: F, t8912: F, t896: F) -> (F,) {
    let t14793 = t14792 * t885;
    let t14800 = 0.5848223622634646207e0 * t2589 * t4940 + 0.5848223622634646207e0 * t896 * t14734 + 0.17315859105681463759e2 * t8912 * t4943 + t14658 - t14685 + 1.0 * t14739 * t886 + 2.0 * t11289 * t1437 + 2.0 * t3822 * t3845 - 2.0 * t8899 * t4892 + 1.0 * t2545 * t4908 + 1.0 * t877 * t14793 + 0.32163958997385070134e2 * t8842 * t4911 - 4.0 * t11366 * t3827;
    (t14800,)
}
