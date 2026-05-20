//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1290/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1290<F: Float>(t120121: F, t120123: F, t123050: F, t123052: F, t123054: F, t123056: F, t123058: F, t123060: F, t123072: F, t31237: F, t31239: F, t33152: F, t33154: F, t8446: F) -> F {
    let t125818 = t8446 + t33152 + t33154 + F::new(4.0) * t123050 + F::new(4.0) * t123052 + F::new(4.0) * t123054 + F::new(4.0) * t123056 + F::new(4.0) * t123058 + F::new(4.0) * t123060 + F::new(4.0) * t123072 + t31237 + t31239 + t120121 + t120123;
    t125818
}
