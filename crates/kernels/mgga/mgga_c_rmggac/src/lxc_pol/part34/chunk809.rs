//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 809/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk809<F: Float>(t70905: F, t74238: F, t74243: F, t76935: F, t76937: F, t76939: F, t76940: F, t76941: F, t76942: F, t76943: F, t76946: F, t76947: F, t76948: F, t76949: F, t76950: F, t76951: F, t76952: F) -> (F,) {
    let t76953 = t76935 - 0.52557918278704101564e-6 * t74238 + t76937 + 0.76860658247009135557e-5 * t74243 - t76939 - t76940 + t76941 + t76942 - t70905 - t76943 - t76946 - t76947 - t76948 + t76949 + t76950 + t76951 - t76952;
    (t76953,)
}
