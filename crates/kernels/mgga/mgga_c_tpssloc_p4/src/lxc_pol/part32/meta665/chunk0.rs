//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2097/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2097<F: Float>(t27495: F, t85964: F, t15702: F, t8038: F, t85822: F, t27563: F, t85639: F, t24826: F, t27502: F, t27558: F, t7368: F, t94490: F) -> (F, F, F, F, F, F) {
    let t94874 = t85964 * t27495;
    let t94881 = t85822 * t8038 * t15702;
    let t94885 = F::cast_from(0.36554090374405031922e-2_f64) * t85639 * t27563;
    let t94889 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27502;
    let t94891 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27558;
    let t94901 = F::cast_from(0.14621636149762012769e-1_f64) * t94490 * t7368;
    (t94874, t94881, t94885, t94889, t94891, t94901)
}
