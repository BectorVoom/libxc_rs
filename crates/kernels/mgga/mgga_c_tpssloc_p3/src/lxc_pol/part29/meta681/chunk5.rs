//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2298/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2298<F: Float>(t1011: F, t5011: F, t11715: F, t491: F, t85964: F, t27488: F, t1209: F, t1216: F, t1235: F, t15018: F, t15620: F, t15625: F, t24762: F, t24812: F, t24813: F, t24814: F, t24815: F, t24833: F, t24834: F, t27406: F, t27470: F, t27471: F, t27489: F, t27490: F, t27496: F, t27497: F, t27501: F, t27507: F, t3494: F, t3509: F, t3604: F, t3610: F, t5068: F, t7373: F, t85963: F, t94875: F) -> (F, F) {
    let t94986 = t5011 * t1011;
    let t95000 = t85964 * t11715 * t491;
    let t95005 = t85964 * t27488;
    let t95026 = -F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t24813 * t1209 * t1235 * t27497 - F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t27496 * t94986 * t1216 - F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t27496 * t27490 * t3494 + F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t27489 * t27490 * t15620 + F::cast_from(0.49348022005446793095e-1_f64) * t85963 * t95000 * t94875 * t15625 - F::cast_from(0.49348022005446793095e-1_f64) * t85963 * t95005 * t94875 * t3509 + F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t24834 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t24833 * t27501 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t24762 + F::cast_from(2.0_f64) * t3604 * t27471 + F::cast_from(4.0_f64) * t3610 * t27470 * t5068 + F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t24814 * t15018 * t24815;
    (t94986, t95026)
}
