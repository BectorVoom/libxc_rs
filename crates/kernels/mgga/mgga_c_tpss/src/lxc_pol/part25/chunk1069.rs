//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1069/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1069<F: Float>(t4923: F, t8749: F, t8752: F, t903: F, t912: F, t3894: F, t3900: F, t2629: F, t4957: F, t3909: F, t3883: F, t3899: F) -> (F, F, F, F, F) {
    let t14690 = t8749 * t4923;
    let t14691 = t8752 * t903;
    let t14692 = t14690 * t14691;
    let t14694 = F::new(0.10254018858216406658e4) * t912 * t14692;
    let t14696 = F::new(0.23392894490538584828e1) * t3894 * t3900;
    let t14698 = F::new(0.5848223622634646207e0) * t2629 * t4957;
    let t14700 = F::new(0.34631718211362927517e2) * t3894 * t3909;
    let t14701 = t3899 * t3883;
    (t14694, t14696, t14698, t14700, t14701)
}
