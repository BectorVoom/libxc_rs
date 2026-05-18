//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 927/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk927<F: Float>(t33877: F, t34075: F, t3: F, t2039: F, t7801: F, t1458: F, t16524: F, t24465: F, t27254: F, t32295: F, t33185: F, t33192: F, t33195: F, t3941: F, t577: F, t7230: F, t7956: F, t8508: F, t8717: F) -> (F, F, F, F) {
    let t34076 = t33877 + t34075;
    let t34077 = t3 * t34076;
    let t34099 = t2039 * t7801;
    let t34102 = F::new(0.45e1) * t34076 * t577 + F::new(0.135e2) * t32295 * t1458 + F::new(27.0) * t27254 * t2039 + F::new(54.0) * t24465 * t7956 + F::new(27.0) * t7230 * t7801 + F::new(27.0) * t16524 * t8717 + F::new(27.0) * t33185 * t8717 + F::new(54.0) * t3941 * t34099 + t33192 + t33195 + t8508;
    (t34076, t34077, t34099, t34102)
}
