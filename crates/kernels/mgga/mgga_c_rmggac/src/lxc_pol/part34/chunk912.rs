//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 912/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk912<F: Float>(t14665: F, t14670: F, t14674: F, t14676: F, t14677: F, t14678: F, t14679: F, t15528: F, t15529: F, t15532: F, t15607: F, t15609: F, t15610: F, t15611: F, t68354: F, t70735: F) -> (F,) {
    let t78623 = -t14665 + t14670 - t14674 + t15528 + t15529 - t70735 - t68354 + t15532 + t15607 - t14676 - t14677 - t14678 - t14679 - t15609 - t15610 - t15611;
    (t78623,)
}
