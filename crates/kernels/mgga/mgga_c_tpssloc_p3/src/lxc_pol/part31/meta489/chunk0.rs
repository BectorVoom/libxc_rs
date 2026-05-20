//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1669/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1669<F: Float>(t1860: F, t26959: F, t2032: F, t22549: F, t23963: F, t23968: F, t23970: F, t23973: F, t23978: F, t23995: F, t23999: F, t26009: F, t26016: F, t26028: F, t26945: F, t26948: F, t26954: F, t6486: F, t7035: F, t7428: F, t7782: F) -> (F, F) {
    let t26960 = t1860 * t26959;
    let t26964 = t26028 * t2032 / F::new(3.0) + t7428 * t7035 / F::new(3.0) + t6486 * t7782 / F::new(3.0) + t1860 * t26945 / F::new(3.0) - F::new(8.0) / F::new(9.0) * t26948 - F::new(8.0) / F::new(9.0) * t23978 + t23995 - F::new(8.0) / F::new(9.0) * t23999 + F::new(10.0) * t23963 * t26009 + F::new(10.0) / F::new(3.0) * t22549 * t26954 + F::new(10.0) / F::new(3.0) * t26016 * t23970 - F::new(8.0) / F::new(9.0) * t26960 + F::new(40.0) / F::new(9.0) * t23968 + F::new(16.0) / F::new(9.0) * t23973;
    (t26960, t26964)
}
