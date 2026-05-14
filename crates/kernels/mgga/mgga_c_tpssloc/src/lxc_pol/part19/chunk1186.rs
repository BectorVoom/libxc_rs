//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1186/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186<F: Float>(t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F, t41912: F, t894: F, t901: F, t41646: F, t41651: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F) -> (F, F, F) {
    let t41925 = -16.0 / 9.0 * t41678 + 8.0 / 9.0 * t41680 + 8.0 / 3.0 * t41682 + 112.0 / 81.0 * t41684 + 40.0 / 9.0 * t41690 - 20.0 / 9.0 * t41695 - 8.0 * t41699 - 2.0 / 3.0 * t41703 - 8.0 / 9.0 * t41707 + 8.0 * t41711 - 8.0 / 3.0 * t41713 - 12.0 * t41717;
    let t41926 = t41912 + t41925;
    let t41927 = t894 * t41926;
    let t41929 = t901 * t41926;
    let t41931 = -0.8585111111111111111e-1 * t41882 - 0.82785e-1 * t41885 - 0.132456e1 * t41887 + 0.22076e0 * t41889 + 0.99342e0 * t41892 + 0.24154e1 * t41646 + 0.72462e1 * t41651 + 0.80513333333333333333e0 * t41680 - 0.20128333333333333334e1 * t41695 - 0.80513333333333333332e0 * t41707 - 0.24154e1 * t41713 - 0.108693e2 * t41717 + 0.258925e1 * t41927 + 0.16504875e0 * t41929;
    (t41927, t41929, t41931)
}
