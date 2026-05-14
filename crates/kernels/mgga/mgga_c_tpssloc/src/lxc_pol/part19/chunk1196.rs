//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1196/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196<F: Float>(t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F, t41927: F, t41929: F) -> (F, F) {
    let t42187 = -0.27545333333333333333e1 * t41678 + 0.41318e1 * t41682 + 0.21424148148148148148e1 * t41684 + 0.68863333333333333334e1 * t41690 - 0.123954e2 * t41699 - 0.103295e1 * t41703 + 0.123954e2 * t41711 + 0.12349037037037037037e1 * t41863 - 0.55570666666666666668e0 * t41865 + 0.55570666666666666666e0 * t41868 - 0.69463333333333333334e0 * t41870 - 0.23154444444444444445e0 * t41872 + 0.27785333333333333333e0 * t41874 + 0.12349037037037037037e0 * t41876;
    let t42203 = -0.10805407407407407407e0 * t41882 - 0.104195e0 * t41885 - 0.166712e1 * t41887 + 0.27785333333333333334e0 * t41889 + 0.125034e1 * t41892 + 0.41318e1 * t41646 + 0.123954e2 * t41651 + 0.13772666666666666666e1 * t41680 - 0.34431666666666666667e1 * t41695 - 0.13772666666666666667e1 * t41707 - 0.41318e1 * t41713 - 0.185931e2 * t41717 + 0.3529725e1 * t41927 + 0.6311625e0 * t41929;
    (t42187, t42203)
}
