//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1054/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1054<F: Float>(t1425: F, t3806: F, t2481: F, t4879: F, t865: F, t8600: F, t4876: F, t2533: F, t4875: F, t2531: F, t3810: F, t14842: F, t14845: F, t14849: F, t14852: F, t14856: F, t14860: F, t14862: F, t14865: F, t2594: F, t2619: F, t8915: F, t8922: F) -> (F, F, F, F, F, F) {
    let t14866 = t1425 * t3806;
    let t14868 = 4.0 * t2481 * t14866;
    let t14869 = t4879 * t865;
    let t14871 = 0.96491876992155210402e2 * t8600 * t14869;
    let t14872 = t4876 * t865;
    let t14874 = 2.0 * t2481 * t14872;
    let t14875 = t4875 * t2533;
    let t14876 = t14875 * t865;
    let t14878 = 0.16081979498692535067e2 * t2531 * t14876;
    let t14879 = t3810 * t3806;
    let t14881 = 0.32163958997385070134e2 * t2531 * t14879;
    let t14882 = -0.10389515463408878255e3 * t8915 * t14842 - 0.11696447245269292414e1 * t2594 * t14845 + 0.17315859105681463759e2 * t2619 * t14849 + 0.34631718211362927518e2 * t2619 * t14852 + 0.10254018858216406658e4 * t8922 * t14856 + t14860 - t14862 - t14865 + t14868 + t14871 + t14874 - t14878 - t14881;
    (t14868, t14871, t14874, t14878, t14881, t14882)
}
