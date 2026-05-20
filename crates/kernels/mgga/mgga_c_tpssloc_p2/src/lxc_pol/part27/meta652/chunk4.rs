//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2278/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278<F: Float>(t1983: F, t23857: F, t7753: F, t24991: F, t6876: F, t12728: F, t1458: F, t1459: F, t16503: F, t1976: F, t1980: F, t23829: F, t24980: F, t26103: F, t4034: F, t4037: F, t574: F, t652: F, t90034: F, t90036: F, t90038: F, t90040: F, t90041: F, t90044: F, t90051: F, t90059: F, t90062: F, t90064: F, t90068: F, t90380: F, t90411: F) -> F {
    let t90418 = F::new(2.0) * t1983 * t7753 * t23857;
    let t90421 = F::new(6.0) * t6876 * t24991;
    let t90422 = t90034 - t90036 - t90038 + t90040 - F::new(4.0) * t90041 * t1459 - F::new(2.0) * t90044 * t1459 - F::new(4.0) * t26103 * t4037 - t90051 - F::new(4.0) * t4034 * t24980 - F::new(2.0) * t652 * t23829 * t1458 - t90059 + t90062 + t90064 + t90068 + (t90380 + t90411) * t574 - F::new(2.0) * t12728 * t1976 + t90418 + t1980 * t16503 + t90421;
    t90422
}
