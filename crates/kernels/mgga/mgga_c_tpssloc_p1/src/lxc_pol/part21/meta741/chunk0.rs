//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2606/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606<F: Float>(t11791: F, t5024: F, t11820: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t11709: F, t15640: F, t1227: F, t13969: F, t15611: F) -> (F, F, F, F, F, F) {
    let t52991 = t5024 * t11791;
    let t52993 = t5002 * t11820;
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53001 = t11709 * t15640;
    let t53023 = t1227 * t13969 * t15611;
    (t52991, t52993, t52995, t52999, t53001, t53023)
}
