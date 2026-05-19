//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 938/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk938<F: Float>(t23110: F, t23185: F, t30685: F, t1880: F, t1894: F, t214: F, t23150: F, t23012: F, t8357: F, t30690: F, t6547: F, t23030: F, t30681: F) -> (F, F, F, F, F) {
    let t112983 = t23185 * t23110 * t30685;
    let t112984 = F::cast_from(0.16449340668482264365e-1_f64) * t112983;
    let t112988 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t214 * t1894 * t23150;
    let t112990 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8357;
    let t112991 = t6547 * t30690;
    let t112992 = F::cast_from(0.76763589786250567036e-1_f64) * t112991;
    let t112995 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30681;
    (t112984, t112988, t112990, t112992, t112995)
}
