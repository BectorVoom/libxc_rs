//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 614/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk614<F: Float>(t14082: F, t14089: F, t4789: F, t68949: F, t3046: F, t880: F, t899: F, t2144: F, t1550: F, t7778: F, t7799: F, t504: F, t7190: F, t14189: F, t16156: F, t13966: F, t2046: F, t7305: F) -> (F, F, F, F, F, F, F) {
    let t69027 = t14089 * t14082 * t4789 * t68949;
    let t69041 = t899 * t880 * t3046;
    let t69045 = t899 * t2144 * t3046;
    let t69049 = t1550 * t7778 * t7799;
    let t69054 = t504 * t7190;
    let t69057 = t16156 * t14189;
    let t69060 = t2046 * t13966 * t7305;
    (t69027, t69041, t69045, t69049, t69054, t69057, t69060)
}
