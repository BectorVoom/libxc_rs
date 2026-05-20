//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1479/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1479<F: Float>(t117687: F, t120807: F, t120809: F, t120818: F, t122804: F, t122806: F, t122808: F, t122817: F, t2039: F, t24972: F, t27170: F, t27281: F, t32406: F, t4072: F, t5376: F, t7235: F, t7423: F, t96311: F, t96334: F) -> F {
    let t125043 = F::new(27.0) * t117687 * t5376 + t120807 + t122804 + t122806 + t122808 + t120809 + F::new(0.135e2) * t96311 * t2039 + t122817 + F::new(27.0) * t96334 * t7235 + F::new(0.135e2) * t7423 * t27170 + t120818 + F::new(27.0) * t24972 * t27281 + F::new(0.135e2) * t32406 * t4072;
    t125043
}
