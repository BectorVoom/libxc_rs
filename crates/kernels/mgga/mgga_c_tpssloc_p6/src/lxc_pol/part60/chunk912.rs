//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 912/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk912<F: Float>(t111: F, t6470: F, t2239: F, t5385: F, t1887: F, t22797: F, t268: F, t547: F, t6559: F, t225: F, t22643: F, t23069: F) -> (F, F, F, F, F, F) {
    let t55388 = t6470 * t111;
    let t55921 = t5385 * t2239;
    let t81159 = t22797 * t1887;
    let t81228 = t6559 * t547 * t268;
    let t81326 = t22643 * t225;
    let t81591 = t23069 * t1887;
    (t55388, t55921, t81159, t81228, t81326, t81591)
}
