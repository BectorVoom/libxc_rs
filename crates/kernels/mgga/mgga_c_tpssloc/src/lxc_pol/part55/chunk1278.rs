//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1278/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1278<F: Float>(t118006: F, t27608: F, t32446: F, t373: F, t471: F, t10401: F, t117949: F, t117954: F, t117977: F, t119243: F, t1734: F, t1748: F, t24685: F, t24727: F, t27629: F, t27636: F, t27638: F, t27644: F, t32429: F, t32433: F, t32448: F, t34263: F, t3500: F, t4950: F, t4954: F, t4979: F, t4988: F, t5030: F, t7337: F, t8028: F) -> (F, F) {
    let t125443 = t27608 * t118006;
    let t125453 = t471 * t32446 * t373;
    let t125459 = -t117949 * t1748 / F::new(2304.0) - t32448 * t5030 / F::new(2304.0) - F::new(0.40372756094140390856e-3) * t24685 * t34263 + F::new(0.80745512188280781712e-3) * t27636 * t24727 * t1734 * t27638 - F::new(0.40372756094140390856e-3) * t27636 * t7337 * t1734 * t27644 - t117977 * t4950 / F::new(2304.0) - t117977 * t4954 / F::new(2304.0) - F::new(0.40372756094140390856e-3) * t125443 - F::new(0.40372756094140390856e-3) * t27629 * t32433 + t3500 * t117954 * t10401 * t119243 * t4979 / F::new(768.0) + F::new(5.0) / F::new(6912.0) * t125453 * t119243 * t4988 + F::new(0.32298204875312312685e-2) * t8028 * t32429;
    (t125453, t125459)
}
