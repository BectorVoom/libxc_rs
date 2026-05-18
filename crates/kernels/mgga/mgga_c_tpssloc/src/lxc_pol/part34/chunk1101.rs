//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1101/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1101<F: Float>(t1887: F, t22797: F, t22715: F, t6887: F, t12225: F, t22641: F, t268: F, t547: F, t6559: F, t22644: F, t81152: F, t1988: F, t81071: F) -> (F, F, F, F, F, F) {
    let t81159 = t22797 * t1887;
    let t81186 = t22715 * t6887;
    let t81195 = t22641 * t12225;
    let t81228 = t6559 * t547 * t268;
    let t81281 = t81152 * t22644;
    let t81317 = t81071 * t1988;
    (t81159, t81186, t81195, t81228, t81281, t81317)
}
