//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1307/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1307<F: Float>(t43776: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F) -> (F, F) {
    let t44249 = 0.16979925925925925926e1 * t43776;
    let t44258 = 0.62517e0 * t43759 - 0.10805407407407407407e0 * t43766 + 0.27785333333333333333e0 * t43768 - 0.166712e1 * t43770 + 0.27785333333333333334e0 * t43773 + t44249 + 0.6311625e0 * t43833 + 0.55570666666666666668e0 * t43835 - 0.166712e1 * t43837 - 0.27785333333333333333e0 * t43839 + 0.55570666666666666666e0 * t43842 - 0.125034e1 * t43845 + 0.250068e1 * t43848 + 0.104195e0 * t43851;
    let t44274 = -0.23154444444444444445e0 * t43855 - 0.12349037037037037037e0 * t43857 - 0.12349037037037037037e1 * t43859 + 0.69463333333333333334e0 * t43861 + 0.13892666666666666667e1 * t43863 - 0.705945e1 * t43866 + 0.1262325e1 * t43869 + 0.158837625e2 * t43872 - 0.94674375e0 * t43875 - 0.6618234375e1 * t43882 + 0.3529725e1 * t43884 - 0.52945875e1 * t43887 + 0.2366859375e0 * t43890 + 0.94674375e0 * t43892;
    (t44258, t44274)
}
