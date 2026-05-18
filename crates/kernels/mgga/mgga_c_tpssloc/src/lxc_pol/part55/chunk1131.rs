//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1131/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1131<F: Float>(t40610: F, t2751: F, t10108: F, t257: F, t3639: F, t11604: F, t496: F, t1406: F, t9238: F, t2239: F, t3951: F, t12461: F, t5356: F) -> (F, F, F, F, F, F, F, F) {
    let t40611 = F::new(1.0) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::new(1.0) / t40771;
    let t40889 = F::new(1.0) / t10108 / t257;
    let t43705 = t3639 * t3639;
    let t43706 = F::new(1.0) / t43705;
    let t45349 = F::new(1.0) / t11604 / t496;
    let t45844 = t1406 * t9238;
    let t46104 = t3951 * t2239;
    let t55242 = t5356 * t12461;
    (t40611, t40772, t40889, t43706, t45349, t45844, t46104, t55242)
}
