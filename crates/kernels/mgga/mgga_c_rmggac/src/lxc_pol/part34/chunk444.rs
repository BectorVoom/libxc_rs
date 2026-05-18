//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 444/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk444<F: Float>(t797: F, t9008: F, t5271: F, t8625: F, t5162: F, t8631: F, t4669: F, t8635: F, t8645: F, t5259: F, t8901: F, t8905: F) -> (F, F, F, F, F, F, F) {
    let t9009 = t797 * t9008;
    let t9011 = t5271 * t8625;
    let t9013 = t5162 * t8631;
    let t9015 = t4669 * t8635;
    let t9017 = t5271 * t8645;
    let t9021 = t5259 * t8901;
    let t9023 = t4669 * t8905;
    (t9009, t9011, t9013, t9015, t9017, t9021, t9023)
}
