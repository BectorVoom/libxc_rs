//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 760/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk760<F: Float>(t13839: F, t2044: F, t570: F, t7554: F, t15347: F, t69836: F, t1653: F, t1986: F, t305: F, t3141: F, t13848: F, t13850: F, t8602: F, t503: F, t551: F, t3157: F) -> (F, F, F, F, F) {
    let t75892 = t13839 * t2044 * t7554 * t570;
    let t75895 = t69836 * t15347;
    let t75907 = t3141 * t1986 * t305 * t1653;
    let t75910 = t8602 * t13848 * t13850;
    let t75920 = t503 * t551;
    let t75921 = t75920 * t3157;
    (t75892, t75895, t75907, t75910, t75921)
}
