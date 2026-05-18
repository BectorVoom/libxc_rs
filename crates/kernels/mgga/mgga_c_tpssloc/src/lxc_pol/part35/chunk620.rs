//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 620/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk620<F: Float>(t5758: F, t932: F, t2888: F, t5742: F, t2892: F, t4335: F, t5679: F, t5683: F, t5687: F, t324: F, t1580: F) -> (F, F, F, F, F) {
    let t5759 = t5758 * t932;
    let t5762 = t5742 * t2888;
    let t5769 = t2892 + F::new(0.61805555555555555556e-2) * t4335 - F::new(0.61805555555555555555e-2) * t5679 + F::new(0.18541666666666666667e-1) * t5683 - F::new(0.92708333333333333333e-2) * t5687;
    let t5770 = t5769 * t324;
    let t5774 = t1580 * t1580;
    (t5759, t5762, t5769, t5770, t5774)
}
