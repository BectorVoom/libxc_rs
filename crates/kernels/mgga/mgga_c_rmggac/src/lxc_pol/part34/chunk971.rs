//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 971/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk971<F: Float>(t68753: F, t68739: F, t71031: F, t74609: F, t74610: F, t74616: F, t74647: F, t74655: F, t77121: F, t77123: F, t77125: F, t77127: F, t77129: F, t77132: F, t77134: F, t77135: F, t77137: F) -> F {
    let t77138 = F::cast_from(0.54549323308490683456e-1_f64) * t68753;
    let t77139 = -t74609 + t77121 - F::cast_from(0.31062809106223861415e-2_f64) * t74610 + t68739 + t77123 - t77125 - t77127 - t77129 - F::cast_from(0.49700494569958178264e-1_f64) * t74616 - t77132 + t71031 + F::cast_from(0.58171619854173713846e-5_f64) * t74647 + t77134 - t77135 + F::cast_from(0.4379826523225341797e-6_f64) * t74655 + t77137 + t77138;
    t77139
}
