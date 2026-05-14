//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 935/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk935<F: Float>(t2613: F, t2621: F, t2542: F, t876: F, t2574: F, t872: F, t2573: F, t301: F, t296: F, t8660: F, t8664: F, t875: F, t2576: F, t304: F, t2586: F, t895: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8833 = t2613 * t2621;
    let t8837 = t2542 * t876;
    let t8842 = t872 * t2574;
    let t8846 = 1.0 / t2573 / t301;
    let t8847 = t296 * t8846;
    let t8871 = 0.16068111111111111111e1 * t8660;
    let t8872 = 0.46308888888888888888e0 * t8664;
    let t8887 = 1.0 / t2573 / t875;
    let t8888 = t296 * t8887;
    let t8890 = 1.0 / t2576 / t304;
    let t8894 = t2586 * t895;
    (t8833, t8837, t8842, t8847, t8871, t8872, t8888, t8890, t8894)
}
