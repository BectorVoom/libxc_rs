//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 995/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk995<F: Float>(t5: F, t25: F, t265: F, t394: F, t27326: F, t27368: F, t112: F, t25882: F, t1409: F, t2116: F, t25398: F, t3966: F, t40: F, t607: F, t7274: F, t7992: F, t1240: F, t1760: F, t2122: F, t1186: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t27370 = piecewise3(t8, 0.0, t27326 + t27368);
    let t27371 = t27370 * t112;
    let t27373 = piecewise3(t395, 0.0, t25882);
    let t27380 = piecewise3(t115, t25398, t7274 * t1409 / 2.0 + t2116 * t3966 / 2.0 + t27373 * t40 / 2.0 + t7992 * t607 / 2.0);
    let t27381 = t1240 * t1760;
    let t27382 = t2122 * t27381;
    let t27383 = t1186 * t27382;
    (t27370, t27371, t27380, t27381, t27383)
}
