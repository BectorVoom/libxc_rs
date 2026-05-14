//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1145/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1145<F: Float>(t81146: F, t154: F, t2690: F, t3748: F, t22691: F, t1887: F, t22797: F, t22715: F, t6887: F, t12225: F, t22641: F, t268: F, t547: F, t6559: F, t12248: F, t2006: F) -> (F, F, F, F, F, F, F, F, F) {
    let t81147 = 0.13707783890401886971e-2 * t81146;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81154 = 0.98696044010893586188e-1 * t81153;
    let t81159 = t22797 * t1887;
    let t81186 = t22715 * t6887;
    let t81195 = t22641 * t12225;
    let t81228 = t6559 * t547 * t268;
    let t81243 = t12248 * t2006;
    (t81147, t81151, t81152, t81154, t81159, t81186, t81195, t81228, t81243)
}
