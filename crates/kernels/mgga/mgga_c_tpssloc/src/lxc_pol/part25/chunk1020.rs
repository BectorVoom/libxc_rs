//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1020/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1020<F: Float>(t23098: F, t81782: F, t81783: F, t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t22690: F, t2379: F, t841: F, t23072: F, t23083: F, t23069: F, t2610: F) -> (F, F, F, F, F) {
    let t81785 = t81782 * t81783 * t23098;
    let t81788 = t22822 * t6589 * t281;
    let t81789 = t81788 * t23124;
    let t81792 = t6597 * t23076 * t281;
    let t81795 = t81792 * t22690 * t841 * t2379;
    let t81797 = t23083 * t23072;
    let t81799 = t23069 * t2610;
    (t81785, t81789, t81795, t81797, t81799)
}
