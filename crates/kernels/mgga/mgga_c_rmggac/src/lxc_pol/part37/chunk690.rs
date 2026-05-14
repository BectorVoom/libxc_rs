//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 690/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk690<F: Float>(t68504: F, t68505: F, t68508: F, t73917: F, t2145: F, t27: F, t649: F, t8794: F, t13888: F, t3133: F, t8581: F, t13862: F, t1603: F, t13819: F, t8346: F, t13823: F, t1665: F, t7755: F) -> (F, F, F, F, F, F) {
    let t74228 = t68504 * t68505 * t73917 * t68508;
    let t74232 = t2145 * t27 * t649 * t8794;
    let t74235 = t3133 * t13888 * t8581;
    let t74238 = t3133 * t13862 * t1603;
    let t74240 = t13819 * t8346;
    let t74243 = t13823 * t7755 * t1665;
    (t74228, t74232, t74235, t74238, t74240, t74243)
}
