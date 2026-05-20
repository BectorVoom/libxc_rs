//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1789/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1789<F: Float>(t23069: F, t805: F, t2628: F, t2633: F, t6605: F, t243: F, t598: F, t213: F, t1894: F, t236: F, t2379: F, t6584: F, t6604: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23070 = t23069 * t805;
    let t23071 = F::new(7.0) / F::new(72.0) * t23070;
    let t23072 = t2628 * t2633;
    let t23073 = t6605 * t23072;
    let t23075 = t243 * t243;
    let t23076 = F::new(1.0) / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23080 = t1894 * t236 * t2379;
    let t23081 = t23078 * t23080;
    let t23083 = t6584 * t6604;
    (t23070, t23071, t23072, t23073, t23075, t23076, t23077, t23080, t23081, t23083)
}
