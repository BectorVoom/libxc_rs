//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2261/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261<F: Float>(t23858: F, t7685: F, t22607: F, t7688: F, t1390: F, t16018: F, t1983: F, t6878: F, t22574: F, t56194: F, t8643: F, t113: F, t1393: F, t1459: F, t26138: F, t4072: F, t5107: F, t6515: F, t652: F, t6862: F, t83935: F, t86673: F, t86676: F, t86679: F, t86682: F, t86684: F, t86688: F, t86693: F, t86698: F, t86700: F, t86702: F, t89836: F, t90016: F) -> F {
    let t90020 = F::cast_from(2.0_f64) * t7685 * t23858;
    let t90022 = F::cast_from(3.0_f64) * t22607 * t7688;
    let t90023 = t1390 * t16018;
    let t90026 = F::cast_from(3.0_f64) * t1983 * t6878 * t90023;
    let t90029 = F::cast_from(6.0_f64) * t22574 * t8643 * t56194;
    let t90030 = F::cast_from(2.0_f64) * t26138 * t1393 + t86673 + t86676 + t86679 - F::cast_from(2.0_f64) * t6515 * t5107 + t86682 - t86684 - t86688 - F::cast_from(2.0_f64) * t83935 * t1459 + t86693 - F::cast_from(4.0_f64) * t652 * t6862 * t4072 - t86698 - t86700 - t86702 - t113 * (t89836 + t90016) + t90020 + t90022 + t90026 - t90029;
    t90030
}
