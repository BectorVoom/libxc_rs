//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1395/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1395<F: Float>(t1873: F, t96311: F, t120826: F, t120830: F, t120835: F, t120836: F, t120838: F, t120840: F, t120848: F, t120851: F, t123261: F, t31287: F, t33192: F, t577: F) -> F {
    let t123306 = t96311 * t1873;
    let t123313 = F::new(0.135e2) * t120826 + F::new(0.135e2) * t123306 + t120830 + t31287 + t120835 + F::new(27.0) * t120836 + F::new(27.0) * t120838 + F::new(27.0) * t120840 + t33192 + t120848 + t120851 + F::new(0.45e1) * t123261 * t577;
    t123313
}
