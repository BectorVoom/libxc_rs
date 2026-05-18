//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 816/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk816<F: Float>(t5: F, t24006: F, t112: F, t1268: F, t12734: F, t12739: F, t2039: F, t2314: F, t2363: F, t23917: F, t23938: F, t23941: F, t5113: F, t671: F, t7042: F, t7056: F, t9348: F) -> (F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t24007 = piecewise3::<f64>(t8, F::new(0.0), t24006);
    let t24008 = t24007 * t112;
    let t24026 = F::new(2.0) * t1268 * t23917 + F::new(4.0) * t12734 * t2039 + F::new(2.0) * t12739 * t2039 + F::new(2.0) * t2039 * t9348 + F::new(4.0) * t2314 * t7056 + F::new(2.0) * t2363 * t7042 + F::new(4.0) * t23938 * t671 + F::new(4.0) * t5113 * t7056 + F::new(2.0) * t23941 + t24008;
    (t24007, t24008, t24026)
}
