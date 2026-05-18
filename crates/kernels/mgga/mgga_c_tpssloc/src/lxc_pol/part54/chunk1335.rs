//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1335/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1335<F: Float>(t31169: F, t5234: F, t31172: F, t114002: F, t32721: F, t16242: F, t31170: F, t5248: F, t550: F, t114011: F, t12419: F, t1307: F, t1336: F, t240: F, t241: F, t5301: F, t552: F) -> (F, F, F, F, F) {
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    let t120344 = t114002 * t32721;
    let t120348 = t31170 * t5248 * t16242 * t550;
    let t120350 = t114011 * t32721;
    let t120357 = t1336 * t552 * t240 * t241 * t12419 * t5301 * t1307;
    (t120342, t120344, t120348, t120350, t120357)
}
