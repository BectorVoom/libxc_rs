//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 651/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk651<F: Float>(t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2617: F, t4162: F, t4166: F, t4281: F, t4283: F, t4286: F, t4288: F, t4291: F, t4292: F, t4296: F, t4298: F, t808: F, t812: F, t861: F, t863: F) -> F {
    let t4300 = t1499 * t863 - t1523 * t2617 + t1525 * t808 + t226 * t4298 + t255 * t4162 - t4166 * t861 + F::cast_from(2.0_f64) * t4281 * t4283 - t4286 * t812 - t4288 * t812 - t4291 * t4292 - t4296 * t812;
    t4300
}
