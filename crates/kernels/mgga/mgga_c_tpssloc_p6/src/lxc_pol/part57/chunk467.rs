//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 467/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk467<F: Float>(t2912: F, t2919: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F) -> F {
    let t5790 = -F::cast_from(0.1294625e1_f64) * t5699 + F::cast_from(0.258925e1_f64) * t5706 + t2912 + F::cast_from(0.20128333333333333334e0_f64) * t4335 - F::cast_from(0.20128333333333333333e0_f64) * t5679 + F::cast_from(0.60385e0_f64) * t5683 - F::cast_from(0.301925e0_f64) * t5687 + F::cast_from(0.82524375e-1_f64) * t5712 + F::cast_from(0.16504875e0_f64) * t5714 + t2919 + F::cast_from(0.11038e0_f64) * t4384 - F::cast_from(0.27595e-1_f64) * t5718 + F::cast_from(0.16557e0_f64) * t5721 - F::cast_from(0.82785e-1_f64) * t5724;
    t5790
}
