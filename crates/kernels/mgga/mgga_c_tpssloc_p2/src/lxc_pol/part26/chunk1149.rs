//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1149/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1149<F: Float>(t134: F, t221: F, t2250: F, t3: F, t3034: F, t371: F, t13487: F, t1877: F, t1915: F, t193: F, t202: F, t23285: F, t23290: F, t23295: F, t2379: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t6666: F, t6670: F, t776: F, t868: F, t870: F) -> (F, F, F, F, F) {
    let t23383 = t221 * t134;
    let t23413 = t3 * t2250;
    let t23508 = F::new(1.0) / t3034 / t371;
    let t23598 = F::new(1.0) / t3034;
    let t23772 = t193 * t202 * t23285 * t870 - F::new(6.0) * t13487 * t2522 * t6670 - F::new(2.0) * t1877 * t23290 * t868 + F::new(2.0) * t1877 * t23295 * t2749 - t1877 * t2745 * t6670 + F::new(6.0) * t1915 * t2379 * t4314 + F::new(3.0) * t1915 * t2522 * t2553 + F::new(6.0) * t2522 * t6666 * t776;
    (t23383, t23413, t23508, t23598, t23772)
}
