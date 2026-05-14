//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1212/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1212<F: Float>(t27188: F, t6535: F, t1437: F, t1862: F, t645: F, t8308: F, t115888: F, t33568: F, t113875: F, t641: F, t1409: F, t83817: F, t31682: F, t3966: F, t8513: F, t8514: F) -> (F, F, F, F, F, F, F) {
    let t121019 = 2.0 * t27188 * t6535;
    let t121022 = t1862 * t1437;
    let t121024 = t8308 * t121022 * t645;
    let t121029 = t115888 * t33568;
    let t121032 = t113875 * t121022 * t641;
    let t121040 = t8308 * t83817 * t1409;
    let t121044 = t8308 * t31682 * t3966;
    let t121050 = t8513 * t8514 * t1409;
    (t121019, t121024, t121029, t121032, t121040, t121044, t121050)
}
