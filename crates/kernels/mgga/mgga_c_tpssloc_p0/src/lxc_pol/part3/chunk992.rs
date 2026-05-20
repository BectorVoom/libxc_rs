//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 992/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk992<F: Float>(t109: F, t12812: F, t1268: F, t12724: F, t12725: F, t12728: F, t12734: F, t12739: F, t1458: F, t2314: F, t2363: F, t4028: F, t4072: F, t5113: F, t671: F, t9348: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t12813 = piecewise3::<F>(t110, F::new(0.0), t12812);
    let t12816 = F::new(2.0) * t1268 * t12813 + F::new(4.0) * t12725 * t671 + F::new(4.0) * t12734 * t1458 + F::new(2.0) * t12739 * t1458 + F::new(2.0) * t1458 * t9348 + F::new(4.0) * t2314 * t4072 + F::new(2.0) * t2363 * t4028 + F::new(4.0) * t4072 * t5113 + t12724 + F::new(2.0) * t12728;
    (t12813, t12816)
}
