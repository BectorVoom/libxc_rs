//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 919/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk919<F: Float>(t1307: F, t212: F, t12225: F, t2586: F, t535: F, t9534: F, t9538: F, t1337: F, t3792: F, t550: F, t1339: F, t836: F) -> (F, F, F, F, F) {
    let t12226 = t212 * t1307;
    let t12227 = t12225 * t12226;
    let t12228 = t2586 * t12227;
    let t12236 = F::cast_from(0.13888888888888888889e-3_f64) * t9534 * t535 * t9538;
    let t12247 = t1337 * t1337;
    let t12248 = F::new(1.0) / t12247;
    let t12250 = t3792 * t550;
    let t12282 = t1339 * t836;
    (t12228, t12236, t12248, t12250, t12282)
}
