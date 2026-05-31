//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1478/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1478<F: Float>(t120800: F, t120803: F, t122776: F, t122780: F, t122784: F, t122786: F, t122788: F, t122790: F, t122794: F, t122800: F, t125024: F, t33195: F, t577: F, t7956: F, t85416: F) -> F {
    let t125029 = t33195 + t122776 + t122780 + F::cast_from(0.45e1_f64) * t125024 * t577 + F::cast_from(27.0_f64) * t85416 * t7956 + t122784 + t122786 + t122788 + t122790 + t120800 + t120803 + t122794 + t122800;
    t125029
}
