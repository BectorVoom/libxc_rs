//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1144/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1144<F: Float>(t14363: F, t324: F, t2924: F, t4475: F, t10632: F, t1580: F, t2906: F, t10756: F, t10820: F, t13729: F, t14257: F, t14329: F, t14332: F, t14337: F, t14344: F, t1581: F, t2856: F, t2900: F, t2925: F, t2930: F, t2933: F, t4434: F, t4449: F, t4472: F, t924: F, t943: F, t952: F) -> (F, F) {
    let t14364 = t14363 * t324;
    let t14366 = t4475 * t2924;
    let t14369 = t1580 * t10632;
    let t14370 = t14369 * t2906;
    let t14373 = F::new(2.0) * t2856 * t4434 + F::new(1.0) * t924 * t14329 + F::new(0.11696447245269292414e1) * t14332 * t952 + F::new(0.5848223622634646207e0) * t4449 * t2925 + F::new(0.17315859105681463759e2) * t14337 * t2933 + F::new(0.5848223622634646207e0) * t10820 * t1581 + F::new(0.11696447245269292414e1) * t2900 * t4472 + F::new(0.5848223622634646207e0) * t943 * t14344 + t13729 + t14257 - F::new(0.19751673498613801407e-1) * t14364 + F::new(0.17315859105681463759e2) * t2930 * t14366 + F::new(0.10254018858216406658e4) * t10756 * t14370;
    (t14364, t14373)
}
