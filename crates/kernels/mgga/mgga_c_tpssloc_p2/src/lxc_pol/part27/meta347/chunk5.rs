//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1445/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1445<F: Float>(t1268: F, t12724: F, t12725: F, t12728: F, t12734: F, t12739: F, t12813: F, t1458: F, t2314: F, t2363: F, t4028: F, t4072: F, t5113: F, t671: F, t9348: F) -> F {
    let t12816 = F::cast_from(2.0_f64) * t1268 * t12813 + F::cast_from(4.0_f64) * t12725 * t671 + F::cast_from(4.0_f64) * t12734 * t1458 + F::cast_from(2.0_f64) * t12739 * t1458 + F::cast_from(2.0_f64) * t1458 * t9348 + F::cast_from(4.0_f64) * t2314 * t4072 + F::cast_from(2.0_f64) * t2363 * t4028 + F::cast_from(4.0_f64) * t4072 * t5113 + t12724 + F::cast_from(2.0_f64) * t12728;
    t12816
}
