//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3137/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137<F: Float>(t43855: F, t43859: F, t43861: F, t43863: F, t44466: F, t50968: F, t50970: F, t50972: F, t50978: F, t64003: F, t64006: F, t64045: F) -> F {
    let t64929 = F::new(4.0) / F::new(3.0) * t64003 - F::new(4.0) * t64006 - t44466 + F::new(5.0) / F::new(81.0) * t43855 + F::new(80.0) / F::new(81.0) * t43859 - F::new(5.0) / F::new(27.0) * t43861 - F::new(10.0) / F::new(27.0) * t43863 - F::new(4.0) / F::new(27.0) * t50968 - F::new(2.0) / F::new(27.0) * t50970 - F::new(4.0) / F::new(9.0) * t50972 + F::new(8.0) / F::new(81.0) * t50978 - t64045 / F::new(6.0);
    t64929
}
