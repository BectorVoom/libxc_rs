//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 973/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk973<F: Float>(t120124: F, t12725: F, t8326: F, t1385: F, t1799: F, t31169: F, t5234: F, t31172: F, t114002: F, t32721: F, t16242: F, t31170: F, t5248: F, t550: F) -> (F, F, F, F, F, F) {
    let t120125 = F::new(2.0) * t120124;
    let t120130 = t12725 * t8326;
    let t120131 = F::new(2.0) * t120130;
    let t120240 = t1799 * t1385;
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    let t120344 = t114002 * t32721;
    let t120348 = t31170 * t5248 * t16242 * t550;
    (t120125, t120131, t120240, t120342, t120344, t120348)
}
