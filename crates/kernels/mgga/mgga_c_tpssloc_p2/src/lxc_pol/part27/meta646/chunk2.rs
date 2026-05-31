//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2221/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2221<F: Float>(t14501: F, t23419: F, t1015: F, t23472: F, t25678: F, t14198: F, t23544: F, t4590: F, t4596: F, t4600: F, t6717: F, t82848: F, t82956: F, t83139: F, t83153: F, t83157: F, t83159: F, t83165: F, t83167: F, t83172: F, t83206: F) -> F {
    let t88704 = t23419 * t14501 / F::cast_from(1728.0_f64);
    let t88723 = F::cast_from(0.20186378047070195428e-3_f64) * t23472 * t1015 * t25678;
    let t88724 = t88704 - F::cast_from(0.20186378047070195428e-3_f64) * t83139 + t6717 * t14198 / F::cast_from(288.0_f64) - t83153 / F::cast_from(162.0_f64) - t83157 / F::cast_from(648.0_f64) - t83159 / F::cast_from(432.0_f64) + t83165 / F::cast_from(864.0_f64) + t83167 / F::cast_from(648.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t83172 + F::cast_from(0.10093189023535097714e-3_f64) * t83206 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t23544 * t4590 - t82956 * t4596 / F::cast_from(72.0_f64) + t82848 * t4600 / F::cast_from(144.0_f64) + t88723;
    t88724
}
