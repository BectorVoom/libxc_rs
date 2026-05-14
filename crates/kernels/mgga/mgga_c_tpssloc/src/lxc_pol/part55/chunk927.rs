//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 927/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk927<F: Float>(t28: F, t1409: F, t2161: F, t25949: F, t27850: F, t3966: F, t52: F, t607: F, t7402: F, t8097: F, t27380: F, t113: F, t24988: F, t24989: F, t24993: F, t24998: F, t25005: F, t25007: F, t25011: F, t25969: F, t25973: F, t27290: F, t27293: F, t27371: F, t510: F, t650: F, t652: F, t8103: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t27857 = piecewise3(t401, t25949, -t7402 * t1409 / 2.0 - t2161 * t3966 / 2.0 + t27850 * t52 / 2.0 - t8097 * t607 / 2.0);
    let t27858 = t27380 + t27857;
    let t27860 = -t113 * t27858 - 2.0 * t27290 * t652 - 2.0 * t27293 * t652 - t27371 * t510 - t650 * t8103 + t24988 + t24989 + t24993 + t24998 - t25005 - t25007 - t25011 - t25969 - t25973;
    (t27858, t27860)
}
