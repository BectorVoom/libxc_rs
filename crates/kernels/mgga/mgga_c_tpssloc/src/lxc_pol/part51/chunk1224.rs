//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1224/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1224<F: Float>(t1799: F, t2018: F, t24432: F, t22574: F, t7685: F, t8644: F, t191: F, t192: F, t7900: F, t2020: F, t7754: F, t8607: F) -> (F, F, F, F, F, F, F) {
    let t33357 = t2018 * t1799;
    let t33358 = t24432 * t33357;
    let t33360 = F::new(3.0) * t22574 * t33358;
    let t33361 = t7685 * t8644;
    let t33363 = t7900 * t191 * t192;
    let t33364 = t33363 * t2020;
    let t33365 = t8607 * t7754;
    (t33357, t33358, t33360, t33361, t33363, t33364, t33365)
}
