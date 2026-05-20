//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1211/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1211<F: Float>(t82046: F, t24255: F, t24256: F, t2613: F, t2617: F, t7104: F, t812: F, t81980: F, t81987: F, t81989: F, t82003: F, t82005: F, t82011: F, t82013: F, t82016: F, t82021: F, t82025: F, t82028: F, t82032: F, t82039: F, t82043: F, t82050: F, t9981: F) -> F {
    let t85027 = F::cast_from(0.55440370401180965083e0_f64) * t82046;
    let t85031 = -F::cast_from(0.69087230807625510332e0_f64) * t81980 - F::cast_from(0.39478417604357434476e0_f64) * t81987 + F::cast_from(0.23029076935875170111e0_f64) * t81989 - F::cast_from(0.16449340668482264365e-1_f64) * t82003 + F::cast_from(0.23029076935875170111e0_f64) * t82005 - F::cast_from(0.38381794893125283518e0_f64) * t82011 - F::cast_from(0.23029076935875170111e0_f64) * t82013 - F::cast_from(0.49348022005446793095e-1_f64) * t82016 - F::cast_from(0.9869604401089358619e-1_f64) * t82021 + F::cast_from(0.9869604401089358619e-1_f64) * t82025 + F::new(6.0) * t812 * t24255 * t9981 + F::cast_from(0.24674011002723396548e-1_f64) * t82028 + F::new(6.0) * t2617 * t24256 - F::cast_from(0.15626873635058151147e0_f64) * t82032 - F::cast_from(0.31253747270116302294e0_f64) * t82039 + F::cast_from(0.16449340668482264365e-1_f64) * t82043 - t85027 + F::cast_from(0.29608813203268075857e0_f64) * t82050 + F::new(3.0) * t2613 * t7104;
    t85031
}
