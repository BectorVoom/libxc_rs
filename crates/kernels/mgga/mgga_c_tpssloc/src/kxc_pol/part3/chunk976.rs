//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 976/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk976<F: Float>(t1406: F, t2239: F, t1437: F, t2241: F, t4021: F, t645: F, t2307: F, t1409: F, t9321: F, t2291: F, t3966: F, t584: F) -> (F, F, F, F, F, F, F) {
    let t12571 = t1406 * t2239;
    let t12582 = t1437 * t2241;
    let t12585 = t4021 * t645;
    let t12588 = t1437 * t2307;
    let t12595 = t9321 * t1409;
    let t12598 = t2291 * t3966;
    let t12603 = F::new(2.0) * t584;
    (t12571, t12582, t12585, t12588, t12595, t12598, t12603)
}
