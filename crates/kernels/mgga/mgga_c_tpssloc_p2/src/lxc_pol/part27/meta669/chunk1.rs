//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2364/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2364<F: Float>(t12550: F, t1442: F, t22461: F, t22619: F, t23829: F, t26103: F, t4028: F, t4073: F, t510: F, t6517: F, t7472: F, t90351: F, t91713: F, t91715: F, t91718: F, t91722: F, t91724: F, t91726: F, t91730: F, t91735: F, t91737: F, t91739: F, t91747: F, t91749: F, t9348: F) -> F {
    let t91750 = -F::new(4.0) * t12550 * t6517 - t1442 * t23829 - F::new(4.0) * t22461 * t4073 - F::new(4.0) * t22619 * t4028 - F::new(4.0) * t26103 * t4073 - t510 * t90351 - F::new(2.0) * t7472 * t9348 - t91713 - t91715 - t91718 - t91722 - t91724 - t91726 - t91730 - t91735 - t91737 - t91739 - t91747 - t91749;
    t91750
}
