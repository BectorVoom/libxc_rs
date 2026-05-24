//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 638/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk638<F: Float>(t3882: F, t904: F, t1448: F, t2621: F, t903: F, t1437: F, t1449: F, t2545: F, t2550: F, t2575: F, t2589: F, t2594: F, t2619: F, t305: F, t3764: F, t3767: F, t3769: F, t3772: F, t3809: F, t3813: F, t3819: F, t3822: F, t3827: F, t3845: F, t3849: F, t3858: F, t3860: F, t3865: F, t877: F, t886: F, t896: F, t905: F) -> (F, F, F, F) {
    let t3883 = t3882 * t904;
    let t3886 = t1448 * t2621;
    let t3887 = t3886 * t903;
    let t3890 = -F::new(0.310907e-1) * t3819 * t305 + F::new(1.0) * t3822 * t886 + F::new(1.0) * t2545 * t1437 - F::new(2.0) * t2550 * t3827 + F::new(1.0) * t877 * t3845 + F::cast_from(0.32163958997385070134e2_f64) * t2575 * t3849 + t3764 - t3767 - t3769 + t3772 - t3809 - t3813 - F::cast_from(0.19751673498613801407e-1_f64) * t3858 + F::cast_from(0.5848223622634646207e0_f64) * t3860 * t905 + F::cast_from(0.5848223622634646207e0_f64) * t2589 * t1449 - F::cast_from(0.11696447245269292414e1_f64) * t2594 * t3865 + F::cast_from(0.5848223622634646207e0_f64) * t896 * t3883 + F::cast_from(0.17315859105681463759e2_f64) * t2619 * t3887;
    (t3883, t3886, t3887, t3890)
}
