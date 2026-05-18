//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1301/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1301<F: Float>(t118365: F, t120809: F, t120811: F, t120818: F, t120820: F, t120823: F, t120830: F, t120835: F, t120848: F, t120851: F, t123294: F, t123296: F, t123298: F, t123306: F, t125966: F, t31287: F, t32643: F, t33192: F, t4072: F, t5376: F, t577: F) -> F {
    let t126015 = t120809 + t120811 + F::new(54.0) * t123294 + F::new(54.0) * t123296 + F::new(0.45e1) * t125966 * t577 + F::new(54.0) * t123298 + t120818 + F::new(27.0) * t118365 * t5376 + t120820 + t120823 + F::new(27.0) * t123306 + t120830 + t31287 + t120835 + F::new(0.135e2) * t32643 * t4072 + t33192 + t120848 + t120851;
    t126015
}
