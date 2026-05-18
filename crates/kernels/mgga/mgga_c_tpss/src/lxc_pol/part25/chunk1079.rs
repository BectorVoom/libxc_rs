//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1079/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1079<F: Float>(t14879: F, t2531: F, t14842: F, t14845: F, t14849: F, t14852: F, t14856: F, t14860: F, t14862: F, t14865: F, t14868: F, t14871: F, t14874: F, t14878: F, t2594: F, t2619: F, t8915: F, t8922: F) -> (F, F) {
    let t14881 = F::new(0.32163958997385070134e2) * t2531 * t14879;
    let t14882 = -F::new(0.10389515463408878255e3) * t8915 * t14842 - F::new(0.11696447245269292414e1) * t2594 * t14845 + F::new(0.17315859105681463759e2) * t2619 * t14849 + F::new(0.34631718211362927518e2) * t2619 * t14852 + F::new(0.10254018858216406658e4) * t8922 * t14856 + t14860 - t14862 - t14865 + t14868 + t14871 + t14874 - t14878 - t14881;
    (t14881, t14882)
}
