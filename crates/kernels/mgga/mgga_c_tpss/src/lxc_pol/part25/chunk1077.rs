//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1077/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1077<F: Float>(t4943: F, t903: F, t4940: F, t2621: F, t4939: F, t3882: F, t3886: F, t4923: F, t8752: F, t11216: F, t3770: F, t10966: F, t3811: F) -> (F, F, F, F, F, F, F) {
    let t14842 = t4943 * t903;
    let t14845 = t4940 * t903;
    let t14848 = t4939 * t2621;
    let t14849 = t14848 * t903;
    let t14852 = t3886 * t3882;
    let t14855 = t4923 * t8752;
    let t14856 = t14855 * t903;
    let t14860 = F::new(4.0) * t11216 * t3770;
    let t14862 = F::new(0.32163958997385070134e2) * t10966 * t3811;
    (t14842, t14845, t14849, t14852, t14856, t14860, t14862)
}
