//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 702/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk702<F: Float>(t1147: F, t1687: F, t1155: F, t1695: F, t3238: F, t3295: F, t3383: F, t3390: F, t4721: F, t4726: F, t4731: F, t4735: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F) {
    let t4835 = t1687 * t1147;
    let t4840 = t1695 * t1155;
    let t4857 = -F::new(0.1294625e1) * t4749 + F::new(0.258925e1) * t4757 + t3383 - F::new(0.10064166666666666667e0) * t3238 - F::new(0.10064166666666666667e0) * t4721 - F::new(0.20128333333333333333e0) * t4726 + F::new(0.60385e0) * t4731 + F::new(0.301925e0) * t4735 + F::new(0.82524375e-1) * t4765 + F::new(0.16504875e0) * t4767 + t3390 - F::new(0.5519e-1) * t3295 - F::new(0.5519e-1) * t4770 - F::new(0.27595e-1) * t4773 + F::new(0.16557e0) * t4776 + F::new(0.82785e-1) * t4779;
    (t4835, t4840, t4857)
}
