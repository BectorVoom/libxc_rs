//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2924/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2924<F: Float>(t10523: F, t2933: F, t5790: F, t959: F, t14662: F, t193: F, t3216: F, t336: F, t4700: F, t4701: F, t59891: F, t59958: F, t59961: F, t59966: F, t59968: F, t59970: F, t59972: F, t60880: F, t60886: F, t60890: F, t60893: F, t60899: F) -> (F, F) {
    let t60903 = F::cast_from(0.10389515463408878255e3_f64) * t959 * t10523 * t5790 * t2933;
    let t60904 = -F::new(2.0) * t193 * t3216 * t336 * t60880 - F::new(2.0) * t14662 * t4700 * t4701 - t59891 + t59958 + t59961 + t59966 + t59968 + t59970 - t59972 + t60886 - t60890 + t60893 - t60899 + t60903;
    (t60903, t60904)
}
