//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1964/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1964<F: Float>(t109: F, t86586: F, t86588: F, t86590: F, t81440: F, t81443: F, t81445: F, t84036: F, t86593: F, t86596: F, t86599: F, t86601: F, t1268: F, t12725: F, t12734: F, t12739: F, t19456: F, t2039: F, t2314: F, t23917: F, t26114: F, t26117: F, t27170: F, t5113: F, t55934: F, t7056: F, t7676: F, t7801: F, t90370: F, t90375: F, t9348: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t92121 = F::new(22.0) / F::new(9.0) * t86586;
    let t92122 = F::new(8.0) / F::new(3.0) * t86588;
    let t92123 = F::new(4.0) / F::new(3.0) * t86590;
    let t92127 = -t84036 - F::new(44.0) / F::new(9.0) * t81440 - F::new(4.0) / F::new(3.0) * t81443 + F::new(2.0) / F::new(3.0) * t81445 - t92121 - t92122 + t92123 - F::new(3.0) / F::new(2.0) * t86593 + t86596 + t86599 / F::new(2.0) - t86601 / F::new(4.0);
    let t92128 = piecewise3::<F>(t110, F::new(0.0), t92127);
    let t92139 = F::new(2.0) * t1268 * t92128 + F::new(4.0) * t12725 * t7056 + F::new(4.0) * t12734 * t7801 + F::new(2.0) * t12739 * t7801 + F::new(4.0) * t19456 * t7056 + F::new(4.0) * t2039 * t55934 + F::new(4.0) * t2039 * t90370 + F::new(2.0) * t2039 * t90375 + F::new(4.0) * t2314 * t27170 + F::new(2.0) * t23917 * t7676 + F::new(4.0) * t26114 * t7056 + F::new(4.0) * t26117 * t7056 + F::new(4.0) * t27170 * t5113 + F::new(2.0) * t7801 * t9348;
    (t92128, t92139)
}
