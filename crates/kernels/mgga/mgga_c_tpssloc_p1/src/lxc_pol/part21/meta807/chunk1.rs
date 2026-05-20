//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2811/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811<F: Float>(t10076: F, t13385: F, t13390: F, t13401: F, t13404: F, t13429: F, t16753: F, t16759: F, t16811: F, t16815: F, t17027: F, t17034: F, t2617: F, t2633: F, t2684: F, t2732: F, t2740: F, t4166: F, t4182: F, t4281: F, t4291: F, t5575: F, t5617: F, t58226: F, t58262: F, t59331: F, t812: F, t829: F) -> F {
    let t59412 = -t10076 * t5617 * t812 - F::new(2.0) * t16753 * t2732 * t812 + F::new(14.0) * t16815 * t2633 * t4281 - t17027 * t2684 * t812 + F::new(8.0) * t4182 * t4281 * t58226 + F::new(4.0) * t4182 * t4281 * t59331 - F::new(2.0) * t4291 * t58262 * t829 + F::new(8.0) * t13385 * t17034 - F::new(4.0) * t13390 * t16759 + F::new(12.0) * t13401 * t17034 + F::new(4.0) * t13404 * t17034 - F::new(2.0) * t13429 * t4166 + F::new(4.0) * t16811 * t2617 + t2740 * t5575;
    t59412
}
